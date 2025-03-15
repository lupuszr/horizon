use std::{str::FromStr, sync::Arc};

use anyhow::anyhow;
use horizon_core::iroh::{
    common::{DocsClient, IrohState},
    document::{AddrInfoOptions, DocTicket, NamespaceId, Query, ShareMode},
};
use wasmtime::component::Resource;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

wasmtime::component::bindgen!({
   world: "extension",
   with: {
      "horizon:extension/network/document": HorizonDocument,
   },
   trappable_imports: true,
   async: true
});
use horizon::extension::network::{Access, Doc};

pub struct HorizonDocument {
    iroh_docs: DocsClient,
}

impl HorizonDocument {
    fn new(iroh_docs: DocsClient) -> Self {
        Self { iroh_docs }
    }
}

pub struct WasmState {
    pub ctx: WasiCtx,
    pub http_ctx: WasiHttpCtx,
    pub table: ResourceTable,
    pub iroh_state: Arc<IrohState>,
}

impl WasiView for WasmState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

impl WasiHttpView for WasmState {
    fn ctx(&mut self) -> &mut wasmtime_wasi_http::WasiHttpCtx {
        &mut self.http_ctx
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl horizon::extension::network::Host for WasmState {}

impl horizon::extension::network::HostDocument for WasmState {
    async fn new(&mut self) -> wasmtime::Result<Resource<HorizonDocument>> {
        let iroh_docs = self.iroh_state.docs.clone();
        let author_id = iroh_docs.authors().create().await?;
        iroh_docs.authors().set_default(author_id).await?;

        let id = self.table.push(HorizonDocument::new(iroh_docs))?;
        Ok(id)
    }
    async fn create(
        &mut self,
        document: wasmtime::component::Resource<HorizonDocument>,
    ) -> wasmtime::Result<Result<Access, ()>> {
        let HorizonDocument { iroh_docs } = self.table.get(&document)?;
        let doc = iroh_docs.create().await?;

        let read_ticket = doc
            .share(ShareMode::Read, AddrInfoOptions::RelayAndAddresses)
            .await?;
        let write_ticket = doc
            .share(ShareMode::Write, AddrInfoOptions::RelayAndAddresses)
            .await?;
        let document_id = doc.id().into_public_key()?.to_string();
        Ok(Ok(Access {
            read: read_ticket.to_string(),
            write: write_ticket.to_string(),
            document_id,
        }))
    }

    async fn load(
        &mut self,
        document: Resource<HorizonDocument>,
        ticket: String,
    ) -> wasmtime::Result<Result<Doc, ()>> {
        let HorizonDocument { iroh_docs } = self.table.get(&document)?;
        let ticket = DocTicket::from_str(ticket.as_str())?;

        let doc = iroh_docs.import(ticket).await?;

        Ok(Ok(Doc {
            document_id: doc.id().to_string(),
        }))
    }
    async fn read_key(
        &mut self,
        document: Resource<HorizonDocument>,
        document_id: String,
        key: String,
    ) -> wasmtime::Result<Vec<u8>> {
        debug_assert!(!document.owned());
        let HorizonDocument { iroh_docs } = self.table.get(&document)?;
        let ns = NamespaceId::from_str(document_id.as_str())?;
        let doc = iroh_docs.open(ns).await?;
        let query = Query::key_exact(key.as_bytes());
        let entry = match doc {
            Some(d) => d.get_one(query).await?,
            None => None,
        };
        println!("DODO");
        let entry = entry.ok_or(anyhow!("Failed to load key"))?;
        return Ok(entry.to_vec());
    }
    async fn add_key_value(
        &mut self,
        document: Resource<HorizonDocument>,
        document_id: String,
        key: String,
        value: Vec<u8>,
    ) -> wasmtime::Result<()> {
        let HorizonDocument { iroh_docs } = self.table.get(&document)?;
        let ns = NamespaceId::from_str(document_id.as_str())?;
        let doc = iroh_docs.open(ns).await?;
        println!("doc:: {:?}", doc);
        let doc = match doc {
            Some(doc) => anyhow::Ok(doc),
            None => Err(anyhow!("No such doc")),
        }?;
        let author_id = iroh_docs.authors().default().await?;
        println!("author id:: {:?}", author_id);
        println!("value:: {:?}", value);
        let key = key.clone();
        doc.set_bytes(author_id, key, value).await?;

        Ok(())
    }

    async fn drop(
        &mut self,
        #[allow(unused_variables)] _: wasmtime::component::Resource<HorizonDocument>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl horizon::extension::logger::Host for WasmState {
    async fn log(&mut self, msg: String) -> wasmtime::Result<()> {
        println!("extension log: {:?}", msg);
        Ok(())
    }
}

pub fn type_annotate_wasi<T, F>(val: F) -> F
where
    F: Fn(&mut T) -> wasmtime_wasi::WasiImpl<&mut T>,
{
    val
}
