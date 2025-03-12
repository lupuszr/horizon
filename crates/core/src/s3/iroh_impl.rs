// use crate::fs::FileSystem;
// use crate::fs::InternalInfo;
// use crate::utils::*;

use futures::stream::{Stream, StreamExt};
use futures::TryStreamExt;
use iroh_docs::{CapabilityKind, NamespaceId};
use std::collections::HashMap;
use stdx::default::default;

use std::path::PathBuf;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::RwLock;

use s3s::dto::*;
use s3s::s3_error;
use s3s::S3ErrorCode;
use s3s::S3Result;
use s3s::S3;
use s3s::{S3Request, S3Response};
use tokio::sync::mpsc::Sender;
use tracing_subscriber::field::debug;

use crate::errors::AppError;
use crate::event::HorizonChannel;
use crate::iroh::common::DocsClient;
use crate::iroh::common::IrohState;

// use std::collections::VecDeque;
// use std::io;
// use std::ops::Neg;
// use std::ops::Not;
// use std::path::Component;
// use std::path::{Path, PathBuf};

// use tokio::fs;
// use tokio::io::AsyncSeekExt;
// use tokio_util::io::ReaderStream;

// use futures::TryStreamExt;
// use md5::{Digest, Md5};
// use numeric_cast::NumericCast;
// use stdx::default::default;
// use tracing::debug;
// use uuid::Uuid;
//

#[derive(Debug)]
struct NamespaceLookupTable {
    forward: HashMap<String, String>,
    reverse: HashMap<String, String>,
}

impl NamespaceLookupTable {
    fn new() -> Self {
        Self {
            forward: HashMap::new(),
            reverse: HashMap::new(),
        }
    }

    fn insert(&mut self, document_id: String, bucket_name: String) {
        self.forward
            .insert(document_id.clone(), bucket_name.clone());
        self.reverse.insert(bucket_name, document_id);
    }

    fn get_by_document_id(&self, key: &str) -> Option<&String> {
        self.forward.get(key)
    }

    fn get_by_bucket_name(&self, value: &str) -> Option<&String> {
        self.reverse.get(value)
    }

    fn remove_by_document_id(&mut self, key: &str) {
        if let Some(value) = self.forward.remove(key) {
            self.reverse.remove(&value);
        }
    }

    fn remove_by_bucket_name(&mut self, value: &str) {
        if let Some(key) = self.reverse.remove(value) {
            self.forward.remove(&key);
        }
    }
}

#[derive(Debug)]
pub struct HorizonSystem {
    iroh_state: IrohState,
    // TODO: we need to find a proper way to store the linking
    namespace_table: RwLock<NamespaceLookupTable>,
}

impl HorizonSystem {
    pub async fn new(
        base_path: &str,
        tx: Sender<HorizonChannel>,
    ) -> Result<HorizonSystem, AppError> {
        let path =
            PathBuf::from_str(base_path).map_err(|err| AppError::PathError(err.to_string()))?;
        let iroh_state = IrohState::new(path, tx).await?;
        Ok(HorizonSystem {
            iroh_state,
            namespace_table: RwLock::new(NamespaceLookupTable::new()),
        })
    }

    pub fn link_bucket_name_with_doc_id(
        &self,
        doc_id: String,
        bucket_name: String,
    ) -> Result<(), AppError> {
        let HorizonSystem {
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
}

#[async_trait::async_trait]
impl S3 for HorizonSystem {
    async fn create_bucket(
        &self,
        req: S3Request<CreateBucketInput>,
    ) -> S3Result<S3Response<CreateBucketOutput>> {
        let input = req.input;
        let bucket_name = input.bucket;
        let HorizonSystem { iroh_state, .. } = self;
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

        self.link_bucket_name_with_doc_id(doc.id().to_string(), bucket_name)
            .map_err(|err| S3ErrorCode::Custom(err.to_string().into()))?;

        let output = CreateBucketOutput {
            ..CreateBucketOutput::default()
        };

        Ok(S3Response::new(output))
    }

    async fn copy_object(
        &self,
        req: S3Request<CopyObjectInput>,
    ) -> S3Result<S3Response<CopyObjectOutput>> {
        todo!()
    }

    async fn delete_bucket(
        &self,
        req: S3Request<DeleteBucketInput>,
    ) -> S3Result<S3Response<DeleteBucketOutput>> {
        todo!()
    }

    async fn delete_object(
        &self,
        req: S3Request<DeleteObjectInput>,
    ) -> S3Result<S3Response<DeleteObjectOutput>> {
        todo!()
    }

    async fn delete_objects(
        &self,
        req: S3Request<DeleteObjectsInput>,
    ) -> S3Result<S3Response<DeleteObjectsOutput>> {
        todo!()
    }

    async fn get_bucket_location(
        &self,
        req: S3Request<GetBucketLocationInput>,
    ) -> S3Result<S3Response<GetBucketLocationOutput>> {
        todo!()
    }

    async fn get_object(
        &self,
        req: S3Request<GetObjectInput>,
    ) -> S3Result<S3Response<GetObjectOutput>> {
        todo!()
    }

    async fn head_bucket(
        &self,
        req: S3Request<HeadBucketInput>,
    ) -> S3Result<S3Response<HeadBucketOutput>> {
        todo!()
    }

    async fn head_object(
        &self,
        req: S3Request<HeadObjectInput>,
    ) -> S3Result<S3Response<HeadObjectOutput>> {
        todo!()
    }

    async fn list_buckets(
        &self,
        _: S3Request<ListBucketsInput>,
    ) -> S3Result<S3Response<ListBucketsOutput>> {
        let HorizonSystem { iroh_state, .. } = self;
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
        req: S3Request<ListObjectsInput>,
    ) -> S3Result<S3Response<ListObjectsOutput>> {
        todo!()
    }

    async fn list_objects_v2(
        &self,
        req: S3Request<ListObjectsV2Input>,
    ) -> S3Result<S3Response<ListObjectsV2Output>> {
        todo!()
    }

    async fn put_object(
        &self,
        req: S3Request<PutObjectInput>,
    ) -> S3Result<S3Response<PutObjectOutput>> {
        let HorizonSystem { iroh_state, .. } = self;
        let IrohState { docs, blobs, .. } = iroh_state;
        let input = req.input;
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
            content_length,
            ..
        } = input;

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

        // pub async fn copy_bytes<S, W>(mut stream: S, writer: &mut W) -> Result<u64>
        // where
        //     S: Stream<Item = Result<Bytes, StdError>> + Unpin,
        //     W: AsyncWrite + Unpin,
        // {
        //     let mut nwritten: u64 = 0;
        //     while let Some(result) = stream.next().await {
        //         let bytes = match result {
        //             Ok(x) => x,
        //             Err(e) => return Err(Error::new(e)),
        //         };
        //         writer.write_all(&bytes).await?;
        //         nwritten += bytes.len() as u64;
        //     }
        //     writer.flush().await?;
        //     Ok(nwritten)
        // }
        while let Some(res) = body.next().await {
            let r = res.unwrap();
        }
        let p = blobs.add_stream(input, tag)

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

        todo!()
    }

    async fn create_multipart_upload(
        &self,
        req: S3Request<CreateMultipartUploadInput>,
    ) -> S3Result<S3Response<CreateMultipartUploadOutput>> {
        todo!()
    }

    async fn upload_part(
        &self,
        req: S3Request<UploadPartInput>,
    ) -> S3Result<S3Response<UploadPartOutput>> {
        todo!()
    }

    async fn upload_part_copy(
        &self,
        req: S3Request<UploadPartCopyInput>,
    ) -> S3Result<S3Response<UploadPartCopyOutput>> {
        todo!()
    }

    async fn list_parts(
        &self,
        req: S3Request<ListPartsInput>,
    ) -> S3Result<S3Response<ListPartsOutput>> {
        todo!()
    }

    async fn complete_multipart_upload(
        &self,
        req: S3Request<CompleteMultipartUploadInput>,
    ) -> S3Result<S3Response<CompleteMultipartUploadOutput>> {
        todo!()
    }

    async fn abort_multipart_upload(
        &self,
        req: S3Request<AbortMultipartUploadInput>,
    ) -> S3Result<S3Response<AbortMultipartUploadOutput>> {
        todo!()
    }
}
