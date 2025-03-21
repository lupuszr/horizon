use futures::stream::StreamExt;
use futures::{Stream, TryStreamExt};
use iroh_blobs::rpc::client::blobs::AddOutcome;
use iroh_docs::engine::LiveEvent;
use iroh_docs::store::Query;
use iroh_docs::{CapabilityKind, DocTicket, NamespaceId};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use stdx::default::default;

use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

use s3s::dto::*;
use s3s::s3_error;
use s3s::S3ErrorCode;
use s3s::S3Result;
use s3s::S3;
use s3s::{S3Request, S3Response};
use tokio::sync::mpsc::Sender;

use crate::errors::AppError;
use crate::event::HorizonChannel;

use crate::iroh::common::IrohState;
use crate::s3::helpers::{adapt_stream, fmt_content_range, reader_to_streaming_blob};

use super::namespace_lookup_table::{NamespaceLookupTable, TicketQuery};

#[derive(Debug, Clone, Deserialize)]
pub enum SharePermission {
    Read,
    Write,
}

#[derive(Debug, Clone)]
pub struct HorizonS3System {
    iroh_state: IrohState,
    // TODO: we need to find a proper way to store the linking
    pub namespace_table: Arc<RwLock<NamespaceLookupTable>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HorizonS3BucketTicket {
    pub bucket_name: String,
    pub ticket: DocTicket,
}

impl HorizonS3System {
    pub async fn new(
        base_path: &str,
        tx: Sender<HorizonChannel>,
    ) -> Result<HorizonS3System, AppError> {
        let path =
            PathBuf::from_str(base_path).map_err(|err| AppError::PathError(err.to_string()))?;
        let iroh_state = IrohState::new(path, tx).await?;

        Ok(HorizonS3System {
            iroh_state,
            namespace_table: Arc::new(RwLock::new(NamespaceLookupTable::new())),
        })
    }

    pub fn new_with_iroh(iroh_state: IrohState) -> HorizonS3System {
        HorizonS3System {
            iroh_state,
            namespace_table: Arc::new(RwLock::new(NamespaceLookupTable::new())),
        }
    }

    // Insert the read ticket for a document by its bucket name
    async fn insert_read_ticket_by_bucket(&self, bucket_name: String) -> Result<(), AppError> {
        let HorizonS3System {
            namespace_table, ..
        } = self;

        // Get the ticket by calling share_bucket with Read permission
        let HorizonS3BucketTicket {
            bucket_name,
            ticket,
        } = self
            .share_bucket(bucket_name, SharePermission::Read)
            .await
            .map_err(|err| AppError::IrohDocsError(err.to_string()))?;

        // Acquire write lock and insert the ticket into the namespace table
        let mut lock = namespace_table.write()?;
        lock.insert_read_ticket_by_bucket(&bucket_name, ticket.to_string())?;

        Ok(())
    }

    // Insert the write ticket for a document by its bucket name
    async fn insert_write_ticket_by_bucket(&self, bucket_name: String) -> Result<(), AppError> {
        let HorizonS3System {
            namespace_table, ..
        } = self;

        // Get the ticket by calling share_bucket with Read permission
        let HorizonS3BucketTicket {
            bucket_name,
            ticket,
        } = self
            .share_bucket(bucket_name, SharePermission::Write)
            .await
            .map_err(|err| AppError::IrohDocsError(err.to_string()))?;

        // Acquire write lock and insert the ticket into the namespace table
        let mut lock = namespace_table.write()?;
        lock.insert_write_ticket_by_bucket(&bucket_name, ticket.to_string())?;

        Ok(())
    }

    pub fn link_bucket_name_with_doc_id(
        &self,
        doc_id: String,
        bucket_name: String,
    ) -> Result<(), AppError> {
        let HorizonS3System {
            namespace_table, ..
        } = self;
        let mut lock = namespace_table.write()?;
        lock.insert(doc_id, bucket_name);
        Ok(())
    }

    pub fn get_id_by_bucket_name(&self, bucket_name: String) -> Result<Option<String>, AppError> {
        let lock = self.namespace_table.read()?;
        let id = (lock.get_by_bucket_name(bucket_name.as_str())).cloned();
        Ok(id)
    }

    pub fn get_bucket_name_by_id(&self, id: String) -> Result<Option<String>, AppError> {
        let lock = self.namespace_table.read()?;
        let id = (lock.get_by_document_id(id.as_str())).cloned();
        Ok(id)
    }

    pub fn sync_buckets(&self) -> Result<(), AppError> {
        let HorizonS3System { iroh_state, .. } = self;
        let IrohState { .. } = iroh_state;

        Ok(())
    }

    pub async fn import_bucket(
        &self,
        bucket_ticket: HorizonS3BucketTicket,
    ) -> Result<impl Stream<Item = anyhow::Result<LiveEvent>>, AppError> {
        let HorizonS3System { iroh_state, .. } = self;
        let IrohState { docs, .. } = iroh_state;
        let HorizonS3BucketTicket {
            bucket_name,
            ticket,
        } = bucket_ticket;

        let document_id = self
            .get_id_by_bucket_name(bucket_name.clone())
            .map_err(|err| AppError::IrohDocsError(err.to_string()))?;
        let None = document_id else {
            return Err(AppError::S3BucketExists);
        };
        let (doc, stream) = docs
            .import_and_subscribe(ticket.clone())
            .await
            .map_err(|err| AppError::IrohDocsError(err.to_string()))?;

        self.link_bucket_name_with_doc_id(doc.id().to_string(), bucket_name)
            .map_err(|err| AppError::IrohDocsError(err.to_string()))?;

        Ok(stream)
    }

    pub async fn share_bucket(
        &self,
        bucket: String,
        permission: SharePermission,
    ) -> Result<HorizonS3BucketTicket, AppError> {
        let HorizonS3System { iroh_state, .. } = self;
        let IrohState { docs, .. } = iroh_state;

        let document_id = self
            .get_id_by_bucket_name(bucket.clone())
            .map_err(|err| AppError::IrohDocsError(err.to_string()))?;
        let Some(document_id) = document_id else {
            return Err(AppError::S3NoBucket);
        };
        let document_id = NamespaceId::from_str(document_id.as_str())
            .map_err(|err| AppError::IrohDocsError(err.to_string()))?;
        let bucket_doc = docs
            .open(document_id)
            .await
            .map_err(|err| AppError::IrohDocsError(err.to_string()))?;
        let Some(bucket_doc) = bucket_doc else {
            return Err(AppError::S3NoBucket);
        };

        let share_mode = match permission {
            SharePermission::Read => iroh_docs::rpc::client::docs::ShareMode::Read,
            SharePermission::Write => iroh_docs::rpc::client::docs::ShareMode::Write,
        };

        let ticket = bucket_doc
            .share(
                share_mode,
                iroh_docs::rpc::AddrInfoOptions::RelayAndAddresses,
            )
            .await
            .map_err(|err| AppError::IrohDocsError(err.to_string()))?;

        Ok(HorizonS3BucketTicket {
            bucket_name: bucket,
            ticket,
        })
    }

    pub async fn share_buckets(
        &self,
        ticket_query: TicketQuery,
        permission: SharePermission,
    ) -> Result<Vec<HorizonS3BucketTicket>, AppError> {
        let HorizonS3System {
            namespace_table, ..
        } = self;

        let ns_table = namespace_table.clone();
        let ns_table = ns_table.read()?;
        let tickets = match permission {
            SharePermission::Read => ns_table.query_read_tickets(ticket_query),
            SharePermission::Write => ns_table.query_read_tickets(ticket_query),
        };

        let tickets: Vec<_> = tickets
            .iter()
            .map(|(k, v)| HorizonS3BucketTicket {
                bucket_name: k.clone(),
                ticket: DocTicket::from_str(v).unwrap(),
            })
            .collect();

        Ok(tickets)
    }

    pub async fn import_buckets(
        &self,
        bucket_tickets: Vec<HorizonS3BucketTicket>,
    ) -> Result<Vec<impl Stream<Item = anyhow::Result<LiveEvent>>>, AppError> {
        let HorizonS3System { iroh_state, .. } = self;
        let IrohState { docs, .. } = iroh_state;

        let mut bucket_import_streams = vec![];

        for bucket_ticket in bucket_tickets.iter() {
            let HorizonS3BucketTicket {
                bucket_name,
                ticket,
            } = bucket_ticket;
            let document_id = self
                .get_id_by_bucket_name(bucket_name.clone())
                .map_err(|err| AppError::IrohDocsError(err.to_string()))?;
            let None = document_id else {
                return Err(AppError::S3BucketExists);
            };
            let (doc, stream) = docs
                .import_and_subscribe(ticket.clone())
                .await
                .map_err(|err| AppError::IrohDocsError(err.to_string()))?;

            self.link_bucket_name_with_doc_id(doc.id().to_string(), bucket_name.clone())
                .map_err(|err| AppError::IrohDocsError(err.to_string()))?;

            bucket_import_streams.push(stream);
        }

        Ok(bucket_import_streams)
    }
}

#[async_trait::async_trait]
impl S3 for HorizonS3System {
    async fn head_bucket(&self, req: S3Request<HeadBucketInput>) -> S3Result<S3Response<HeadBucketOutput>> {
        let input = req.input;
        let bucket_name = input.bucket;
        let HorizonS3System { iroh_state, .. } = self;
        let IrohState { docs, .. } = iroh_state;

        if self
            .get_id_by_bucket_name(bucket_name.clone())
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?
            .is_none()
        {
            return Err(s3_error!(NoSuchBucket));
        }

        Ok(S3Response::new(HeadBucketOutput::default()))
    }

    async fn create_bucket(
        &self,
        req: S3Request<CreateBucketInput>,
    ) -> S3Result<S3Response<CreateBucketOutput>> {
        let input = req.input;
        let bucket_name = input.bucket;
        let HorizonS3System { iroh_state, .. } = self;
        let IrohState { docs, .. } = iroh_state;

        if self
            .get_id_by_bucket_name(bucket_name.clone())
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?
            .is_some()
        {
            return Err(s3_error!(BucketAlreadyExists));
        }

        let doc = docs
            .create()
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;

        self.link_bucket_name_with_doc_id(doc.id().to_string(), bucket_name.clone())
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        self.insert_read_ticket_by_bucket(bucket_name.clone())
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        self.insert_write_ticket_by_bucket(bucket_name)
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;

        let output = CreateBucketOutput {
            ..CreateBucketOutput::default()
        };

        Ok(S3Response::new(output))
    }

    async fn copy_object(
        &self,
        _req: S3Request<CopyObjectInput>,
    ) -> S3Result<S3Response<CopyObjectOutput>> {
        todo!()
    }

    async fn delete_bucket(
        &self,
        req: S3Request<DeleteBucketInput>,
    ) -> S3Result<S3Response<DeleteBucketOutput>> {
        let HorizonS3System { iroh_state, .. } = self;
        let IrohState { docs, .. } = iroh_state;
        let input = req.input;
        let bucket = input.bucket;

        let document_id = self
            .get_id_by_bucket_name(bucket.clone())
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        let Some(document_id) = document_id else {
            return Err(s3_error!(NoSuchBucket));
        };
        let document_id = NamespaceId::from_str(document_id.as_str())
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        docs.drop_doc(document_id)
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;

        let output = DeleteBucketOutput {
            ..Default::default()
        };
        Ok(S3Response::new(output))
    }

    async fn delete_object(
        &self,
        req: S3Request<DeleteObjectInput>,
    ) -> S3Result<S3Response<DeleteObjectOutput>> {
        let HorizonS3System { iroh_state, .. } = self;
        let IrohState { docs, blobs, .. } = iroh_state;
        let input = req.input;
        let bucket = input.bucket;
        let key = input.key;

        let author_id = docs
            .authors()
            .default()
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;

        let document_id = self
            .get_id_by_bucket_name(bucket.clone())
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        let Some(document_id) = document_id else {
            return Err(s3_error!(NoSuchBucket));
        };
        let document_id = NamespaceId::from_str(document_id.as_str())
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        let bucket_doc = docs
            .open(document_id)
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        let Some(bucket_doc) = bucket_doc else {
            return Err(s3_error!(NoSuchBucket));
        };

        let query = format!("object::{}", key);
        let object_query = Query::key_exact(query.clone());
        let entry = bucket_doc
            .get_one(object_query)
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        let Some(entry) = entry else {
            return Err(s3_error!(NoSuchKey));
        };
        let blob_hash = entry.content_hash();

        // we need to remove the blob and the key as well
        bucket_doc
            .del(author_id, query.into_bytes())
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        let _ = blobs.delete_blob(blob_hash).await;

        let output = DeleteObjectOutput {
            ..Default::default()
        };

        Ok(S3Response::new(output))
    }

    async fn delete_objects(
        &self,
        _req: S3Request<DeleteObjectsInput>,
    ) -> S3Result<S3Response<DeleteObjectsOutput>> {
        todo!()
    }

    async fn get_bucket_location(
        &self,
        _req: S3Request<GetBucketLocationInput>,
    ) -> S3Result<S3Response<GetBucketLocationOutput>> {
        todo!()
    }

    async fn get_object(
        &self,
        req: S3Request<GetObjectInput>,
    ) -> S3Result<S3Response<GetObjectOutput>> {
        let HorizonS3System { iroh_state, .. } = self;
        let IrohState { docs, blobs, .. } = iroh_state;
        let input = req.input;
        let bucket = input.bucket;
        let key = input.key;

        let document_id = self
            .get_id_by_bucket_name(bucket.clone())
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        let Some(document_id) = document_id else {
            return Err(s3_error!(NoSuchBucket));
        };
        let document_id = NamespaceId::from_str(document_id.as_str())
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        let bucket_doc = docs
            .open(document_id)
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        let Some(bucket_doc) = bucket_doc else {
            return Err(s3_error!(NoSuchBucket));
        };

        let object_query = Query::key_exact(format!("object::{}", key));
        let entry = bucket_doc
            .get_one(object_query)
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;

        let Some(entry) = entry else {
            return Err(s3_error!(NoSuchKey));
        };

        let hash = entry.content_hash();
        let file_len = entry.content_len();
        let (content_length, content_range) = match input.range {
            None => (file_len, None),
            Some(range) => {
                let file_range = range.check(file_len)?;
                let content_length = file_range.end - file_range.start;
                let content_range =
                    fmt_content_range(file_range.start, file_range.end - 1, file_len);
                (content_length, Some(content_range))
            }
        };
        let reader = blobs
            .read(hash)
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;

        let blob = reader_to_streaming_blob(reader);

        let metadata_query = Query::key_exact(format!("metadata::{}", key));
        let metadata_entry = bucket_doc
            .get_one(metadata_query)
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;

        let object_metadata = if let Some(meta) = metadata_entry {
            let metadata_content = blobs
                .read_to_bytes(meta.content_hash())
                .await
                .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
            let deserialized: HashMap<String, String> =
                serde_cbor::from_slice(&metadata_content).unwrap();

            Some(deserialized)
        } else {
            None
        };

        let output = GetObjectOutput {
            body: Some(blob),
            content_length: Some(content_length as i64),
            content_range,
            // last_modified: Some(last_modified),
            metadata: object_metadata,
            // e_tag: Some(e_tag),
            // checksum_crc32: checksum.checksum_crc32,
            // checksum_crc32c: checksum.checksum_crc32c,
            // checksum_sha1: checksum.checksum_sha1,
            // checksum_sha256: checksum.checksum_sha256,
            ..Default::default()
        };
        Ok(S3Response::new(output))
    }

    async fn head_bucket(
        &self,
        _req: S3Request<HeadBucketInput>,
    ) -> S3Result<S3Response<HeadBucketOutput>> {
        todo!()
    }

    async fn head_object(
        &self,
        _req: S3Request<HeadObjectInput>,
    ) -> S3Result<S3Response<HeadObjectOutput>> {
        todo!()
    }

    async fn list_buckets(
        &self,
        _: S3Request<ListBucketsInput>,
    ) -> S3Result<S3Response<ListBucketsOutput>> {
        let HorizonS3System { iroh_state, .. } = self;
        let IrohState { docs, .. } = iroh_state;

        let mut buckets: Vec<Bucket> = Vec::new();
        let documents: Vec<(NamespaceId, CapabilityKind)> = docs
            .list()
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?
            .try_collect()
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;

        for (id, _kind) in documents {
            let name = self
                .get_bucket_name_by_id(id.to_string())
                .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;

            let bucket = Bucket {
                name,
                ..Default::default()
            };

            buckets.push(bucket);
        }

        let output = ListBucketsOutput {
            buckets: Some(buckets),
            owner: None,
            ..Default::default()
        };
        Ok(S3Response::new(output))
    }

    async fn list_objects(
        &self,
        _req: S3Request<ListObjectsInput>,
    ) -> S3Result<S3Response<ListObjectsOutput>> {
        todo!()
    }

    async fn list_objects_v2(
        &self,
        req: S3Request<ListObjectsV2Input>,
    ) -> S3Result<S3Response<ListObjectsV2Output>> {
        let HorizonS3System { iroh_state, .. } = self;
        let IrohState { docs, .. } = iroh_state;
        let input = req.input;
        let bucket = input.bucket;

        let document_id = self
            .get_id_by_bucket_name(bucket.clone())
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        let Some(document_id) = document_id else {
            return Err(s3_error!(NoSuchBucket));
        };
        let document_id = NamespaceId::from_str(document_id.as_str())
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        let bucket_doc = docs
            .open(document_id)
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        let Some(bucket_doc) = bucket_doc else {
            return Err(s3_error!(NoSuchBucket));
        };

        let object_query = Query::key_prefix("object");
        let entries = bucket_doc
            .get_many(object_query)
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;

        // TODO: read from multiple sources
        let entries = entries.and_then(async |entry| {
            let key = entry.key();
            let key = String::from_utf8_lossy(key);
            let key = key.strip_prefix("object::").unwrap();

            let object = Object {
                key: Some(key.to_string()),
                ..Default::default()
            };
            Ok(object)
        });
        let entries = entries.collect::<Vec<_>>().await;
        let objects = entries
            .into_iter()
            .collect::<anyhow::Result<Vec<_>>>()
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;

        let key_count: i32 = objects.len() as i32;

        let output = ListObjectsV2Output {
            key_count: Some(key_count),
            max_keys: Some(key_count),
            contents: Some(objects),
            delimiter: input.delimiter,
            encoding_type: input.encoding_type,
            name: Some(bucket),
            prefix: input.prefix,
            ..Default::default()
        };
        Ok(S3Response::new(output))
    }

    async fn put_object(
        &self,
        req: S3Request<PutObjectInput>,
    ) -> S3Result<S3Response<PutObjectOutput>> {
        let HorizonS3System { iroh_state, .. } = self;
        let IrohState { docs, blobs, .. } = iroh_state;
        let input = req.input;
        // TODO: add support for other storage class later`
        if let Some(ref storage_class) = input.storage_class {
            let is_valid = ["STANDARD", "REDUCED_REDUNDANCY"].contains(&storage_class.as_str());
            if !is_valid {
                return Err(s3_error!(InvalidStorageClass));
            }
        }

        let PutObjectInput {
            body,
            bucket,
            key,
            metadata,
            ..
        } = input;

        let document_id = self
            .get_id_by_bucket_name(bucket)
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        let Some(document_id) = document_id else {
            return Err(s3_error!(NoSuchBucket));
        };
        let document_id = NamespaceId::from_str(document_id.as_str())
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        let bucket_doc = docs
            .open(document_id)
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        let Some(bucket_doc) = bucket_doc else {
            return Err(s3_error!(NoSuchBucket));
        };
        let Some(body) = body else {
            return Err(s3_error!(IncompleteBody));
        };

        let mut checksum: s3s::checksum::ChecksumHasher = default();
        if input.checksum_crc32.is_some() {
            checksum.crc32 = Some(default());
        }
        if input.checksum_crc32c.is_some() {
            checksum.crc32c = Some(default());
        }
        if input.checksum_sha1.is_some() {
            checksum.sha1 = Some(default());
        }
        if input.checksum_sha256.is_some() {
            checksum.sha256 = Some(default());
        }

        let progress = blobs
            .add_stream(adapt_stream(body), iroh_blobs::util::SetTagOption::Auto)
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;

        let outcome: AddOutcome = progress
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;

        let author_id = docs
            .authors()
            .default()
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        // Link the blob the the doc that represents the specified bucket
        bucket_doc
            .set_hash(
                author_id,
                format!("object::{}", key.clone()),
                outcome.hash,
                outcome.size,
            )
            .await
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        // if there is some metadata link it as well
        if let Some(data) = metadata {
            let serialized = serde_cbor::to_vec(&data)
                .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;

            bucket_doc
                .set_bytes(author_id, format!("metadata::{}", key), serialized)
                .await
                .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;
        }

        // let mut md5_hash = <Md5 as Digest>::new();
        // let stream = body.inspect_ok(|bytes| {
        //     md5_hash.update(bytes.as_ref());
        //     checksum.update(bytes.as_ref());
        // });

        // let checksum = checksum.finalize();
        // if checksum.checksum_crc32 != input.checksum_crc32 {
        //     return Err(s3_error!(BadDigest, "checksum_crc32 mismatch"));
        // }
        // if checksum.checksum_crc32c != input.checksum_crc32c {
        //     return Err(s3_error!(BadDigest, "checksum_crc32c mismatch"));
        // }
        // if checksum.checksum_sha1 != input.checksum_sha1 {
        //     return Err(s3_error!(BadDigest, "checksum_sha1 mismatch"));
        // }
        // if checksum.checksum_sha256 != input.checksum_sha256 {
        //     return Err(s3_error!(BadDigest, "checksum_sha256 mismatch"));
        // }
        //
        //

        let output = PutObjectOutput {
            // e_tag: Some(e_tag),
            // checksum_crc32: checksum.checksum_crc32,
            // checksum_crc32c: checksum.checksum_crc32c,
            // checksum_sha1: checksum.checksum_sha1,
            // checksum_sha256: checksum.checksum_sha256,
            ..Default::default()
        };
        Ok(S3Response::new(output))
    }

    async fn create_multipart_upload(
        &self,
        _req: S3Request<CreateMultipartUploadInput>,
    ) -> S3Result<S3Response<CreateMultipartUploadOutput>> {
        todo!()
    }

    async fn upload_part(
        &self,
        _req: S3Request<UploadPartInput>,
    ) -> S3Result<S3Response<UploadPartOutput>> {
        todo!()
    }

    async fn upload_part_copy(
        &self,
        _req: S3Request<UploadPartCopyInput>,
    ) -> S3Result<S3Response<UploadPartCopyOutput>> {
        todo!()
    }

    async fn list_parts(
        &self,
        _req: S3Request<ListPartsInput>,
    ) -> S3Result<S3Response<ListPartsOutput>> {
        todo!()
    }

    async fn complete_multipart_upload(
        &self,
        _req: S3Request<CompleteMultipartUploadInput>,
    ) -> S3Result<S3Response<CompleteMultipartUploadOutput>> {
        todo!()
    }

    async fn abort_multipart_upload(
        &self,
        _req: S3Request<AbortMultipartUploadInput>,
    ) -> S3Result<S3Response<AbortMultipartUploadOutput>> {
        todo!()
    }
}
