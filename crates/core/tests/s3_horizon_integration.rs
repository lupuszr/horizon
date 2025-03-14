use aws_sdk_s3::operation::create_bucket;
use horizon_core::s3::iroh_impl::HorizonSystem;
use s3s::auth::SimpleAuth;
use s3s::host::SingleDomain;
use s3s::service::S3ServiceBuilder;
use s3s::service::SharedS3Service;
use tokio::sync::mpsc;

use std::env;
use std::fs;

use aws_config::SdkConfig;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::config::Region;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
// use aws_sdk_s3::Client;

use aws_sdk_s3::types::BucketLocationConstraint;
use aws_sdk_s3::types::ChecksumMode;
use aws_sdk_s3::types::CompletedMultipartUpload;
use aws_sdk_s3::types::CompletedPart;
use aws_sdk_s3::types::CreateBucketConfiguration;

use anyhow::Result;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;
use tracing::{debug, error};
use uuid::Uuid;

const FS_ROOT: &str = concat!(env!("CARGO_TARGET_TMPDIR"), "/s3s-fs-tests-aws");
const DOMAIN_NAME: &str = "localhost:8014";
const REGION: &str = "us-west-2";

async fn config() -> SdkConfig {
    // Fake credentials
    let cred = Credentials::for_tests();

    // Setup S3 provider
    fs::create_dir_all(FS_ROOT).unwrap();
    let (tx_send, _rx_sender) = mpsc::channel(100);

    let hs = HorizonSystem::new(FS_ROOT, tx_send).await.unwrap();

    // Setup S3 service
    let service = {
        let mut b = S3ServiceBuilder::new(hs);
        b.set_auth(SimpleAuth::from_single(
            cred.access_key_id(),
            cred.secret_access_key(),
        ));
        b.set_host(SingleDomain::new(DOMAIN_NAME).unwrap());
        b.build()
    };

    // Convert to aws http client
    let se: s3s::service::SharedS3Service = service.into_shared();
    let client = s3s_aws::Client::from(se);

    // Setup aws sdk config
    SdkConfig::builder()
        .credentials_provider(SharedCredentialsProvider::new(cred))
        .http_client(client)
        .region(Region::new(REGION))
        .endpoint_url(format!("http://{DOMAIN_NAME}"))
        .build()
}

async fn create_bucket(c: &Client, bucket: &str) -> Result<()> {
    let location = BucketLocationConstraint::from(REGION);
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(location)
        .build();

    c.create_bucket()
        .create_bucket_configuration(cfg)
        .bucket(bucket)
        .send()
        .await?;

    debug!("created bucket: {bucket:?}");
    Ok(())
}

async fn delete_object(c: &Client, bucket: &str, key: &str) -> Result<()> {
    c.delete_object().bucket(bucket).key(key).send().await?;
    Ok(())
}

async fn delete_bucket(c: &Client, bucket: &str) -> Result<()> {
    c.delete_bucket().bucket(bucket).send().await?;
    Ok(())
}

#[tokio::test]
async fn test_list_buckets() -> Result<()> {
    let cfg = config().await;
    let c = Client::new(&cfg);
    let response1 = c.list_buckets().send().await;
    drop(response1);

    let bucket1 = format!("test-list-buckets-1-{}", Uuid::new_v4());
    let bucket1_str = bucket1.as_str();
    let bucket2 = format!("test-list-buckets-2-{}", Uuid::new_v4());
    let bucket2_str = bucket2.as_str();

    create_bucket(&c, bucket1_str).await?;
    create_bucket(&c, bucket2_str).await?;

    let response2 = c.list_buckets().send().await.unwrap();
    println!("resp2:: {:?}", response2);
    let bucket_names: Vec<_> = response2
        .buckets()
        .iter()
        .filter_map(|bucket| bucket.name())
        .collect();
    assert!(bucket_names.contains(&bucket1_str));
    assert!(bucket_names.contains(&bucket2_str));

    Ok(())
}

#[tokio::test]
async fn test_list_objects_v2() -> Result<()> {
    let cfg = config().await;
    let c = Client::new(&cfg);
    let bucket = format!("test-list-objects-v2-{}", Uuid::new_v4());
    let bucket_str = bucket.as_str();
    create_bucket(&c, bucket_str).await?;

    let test_prefix = "/this/is/a/test/";
    let key1 = "this/is/a/test/path/file1.txt";
    let key2 = "this/is/a/test/path/file2.txt";
    {
        let content = "hello world\nनमस्ते दुनिया\n";
        let crc32c = base64_simd::STANDARD
            .encode_to_string(crc32c::crc32c(content.as_bytes()).to_be_bytes());
        c.put_object()
            .bucket(bucket_str)
            .key(key1)
            .body(ByteStream::from_static(content.as_bytes()))
            .checksum_crc32_c(crc32c.as_str())
            .send()
            .await?;
        c.put_object()
            .bucket(bucket_str)
            .key(key2)
            .body(ByteStream::from_static(content.as_bytes()))
            .checksum_crc32_c(crc32c.as_str())
            .send()
            .await?;
    }

    let result = c
        .list_objects_v2()
        .bucket(bucket_str)
        .prefix(test_prefix)
        .send()
        .await;

    let response = result.unwrap();

    let contents: Vec<_> = response
        .contents()
        .iter()
        .filter_map(|obj| obj.key())
        .collect();

    assert!(!contents.is_empty());
    assert!(contents.contains(&key1));
    assert!(contents.contains(&key2));

    Ok(())
}

#[tokio::test]
async fn test_single_object() -> Result<()> {
    let cfg = config().await;
    let c = Client::new(&cfg);

    let bucket = format!("test-single-object-{}", Uuid::new_v4());
    let bucket = bucket.as_str();
    let key = "sample.txt";
    let content = "hello world\n你好世界\n";
    let crc32c =
        base64_simd::STANDARD.encode_to_string(crc32c::crc32c(content.as_bytes()).to_be_bytes());

    create_bucket(&c, bucket).await?;

    {
        let body = ByteStream::from_static(content.as_bytes());
        c.put_object()
            .bucket(bucket)
            .key(key)
            .body(body)
            .metadata("meta", "pig")
            .checksum_crc32_c(crc32c.as_str())
            .send()
            .await?;
    }

    {
        let ans = c
            .get_object()
            .bucket(bucket)
            .key(key)
            .checksum_mode(ChecksumMode::Enabled)
            .send()
            .await?;

        let content_length: usize = ans.content_length().unwrap().try_into().unwrap();
        // let checksum_crc32c = ans.checksum_crc32_c.unwrap();
        let body = ans.body.collect().await?.into_bytes();
        let metadata = ans.metadata.unwrap();

        assert_eq!(content_length, content.len());
        assert_eq!(metadata.get("meta"), Some(&("pig".to_string())));
        // assert_eq!(checksum_crc32c, crc32c);
        assert_eq!(body.as_ref(), content.as_bytes());
    }

    // {
    //     delete_object(&c, bucket, key).await?;
    //     delete_bucket(&c, bucket).await?;
    // }

    Ok(())
}
