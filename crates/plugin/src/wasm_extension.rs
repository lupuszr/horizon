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

pub struct HorizonDocument {}

pub struct WasmState {
    pub ctx: WasiCtx,
    pub http_ctx: WasiHttpCtx,
    pub table: ResourceTable,
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
        let id = self.table.push(HorizonDocument {})?;
        Ok(id)
    }
    async fn read_key(
        &mut self,
        document: Resource<HorizonDocument>,
        key: String,
    ) -> wasmtime::Result<Vec<u8>> {
        debug_assert!(!document.owned());
        println!("DODO");
        return Ok(vec![1, 2]);
    }
    async fn add_key_value(
        &mut self,
        document: Resource<HorizonDocument>,
        key: String,
        value: Vec<u8>,
    ) -> wasmtime::Result<()> {
        todo!()
    }
    async fn drop(
        &mut self,
        rep: wasmtime::component::Resource<HorizonDocument>,
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
