use futures::StreamExt;
use horizon_core::s3::iroh_impl::HorizonSystem;
use s3s::auth::SimpleAuth;
use s3s::host::SingleDomain;
use s3s::service::S3ServiceBuilder;
use tokio::sync::mpsc;

use std::env;
use std::fs;
use std::time::Duration;

use aws_config::SdkConfig;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::config::Region;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
// use aws_sdk_s3::Client;

use aws_sdk_s3::types::BucketLocationConstraint;
use aws_sdk_s3::types::ChecksumMode;
use aws_sdk_s3::types::CreateBucketConfiguration;

use anyhow::Result;
use tracing::debug;
use uuid::Uuid;

const FS_ROOT: &str = concat!(env!("CARGO_TARGET_TMPDIR"), "/s3s-fs-tests-aws");
const DOMAIN_NAME: &str = "localhost:8014";
const REGION: &str = "us-west-2";

async fn config(node_path: &str) -> (SdkConfig, HorizonSystem) {
    // Fake credentials
    let cred = Credentials::for_tests();

    // Setup S3 provider
    fs::create_dir_all(FS_ROOT).unwrap();
    let (tx_send, _rx_sender) = mpsc::channel(100);

    let hs = HorizonSystem::new(node_path, tx_send).await.unwrap();

    // Setup S3 service
    let service = {
        let mut b = S3ServiceBuilder::new(hs.clone());
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
    (
        SdkConfig::builder()
            .credentials_provider(SharedCredentialsProvider::new(cred))
            .http_client(client)
            .region(Region::new(REGION))
            .endpoint_url(format!("http://{DOMAIN_NAME}"))
            .build(),
        hs,
    )
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
    const NODE_FS_ROOT: &str = concat!(env!("CARGO_TARGET_TMPDIR"), "/s3s-fs-test_list_buckets");
    let (cfg, _) = config(NODE_FS_ROOT).await;
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

    {
        fs::remove_dir_all(NODE_FS_ROOT).unwrap();
    }

    Ok(())
}

#[tokio::test]
async fn test_list_objects_v2() -> Result<()> {
    const NODE_FS_ROOT: &str = concat!(env!("CARGO_TARGET_TMPDIR"), "/s3s-fs-test_list_objects_v2");
    let (cfg, _) = config(NODE_FS_ROOT).await;
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

    {
        fs::remove_dir_all(NODE_FS_ROOT).unwrap();
    }

    Ok(())
}

#[tokio::test]
async fn test_single_object() -> Result<()> {
    const NODE_FS_ROOT: &str = concat!(env!("CARGO_TARGET_TMPDIR"), "/s3s-fs-test_single_object");
    let (cfg, _) = config(NODE_FS_ROOT).await;
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

    {
        delete_object(&c, bucket, key).await?;
        delete_bucket(&c, bucket).await?;
        fs::remove_dir_all(NODE_FS_ROOT).unwrap();
    }

    Ok(())
}

/// Tests that an object uploaded to one node in a multi-node S3-like storage system
/// can be correctly retrieved from another node after synchronization.
///
/// ## Test Workflow:
/// 1. Create two independent storage nodes with separate file system roots.
/// 2. Initialize S3 clients (`client1` and `client2`) for each node.
/// 3. Create a unique bucket on `client1` and upload a sample object (`sample.txt`).
/// 4. Verify that the object is stored with the correct metadata and checksum.
/// 5. Share the bucket from `node1` to `node2` using a ticket-based mechanism.
/// 6. Import the shared bucket on `node2` and wait for synchronization to complete.
/// 7. Retrieve the object from `client2` and validate:
///    - Content length matches.
///    - Metadata is correctly transferred.
///    - Checksum integrity is preserved.
#[tokio::test]
async fn test_multinode_object_sync_and_fetch() -> Result<()> {
    const NODE1_FS_ROOT: &str = concat!(env!("CARGO_TARGET_TMPDIR"), "/s3s-fs-tests-node-1a");
    const NODE2_FS_ROOT2: &str = concat!(env!("CARGO_TARGET_TMPDIR"), "/s3s-fs-tests-node-2a");
    let (node1_cfg, node1_handle) = config(NODE1_FS_ROOT).await;
    let client1 = Client::new(&node1_cfg);

    let (node2_cfg, node2_handle) = config(NODE2_FS_ROOT2).await;
    let client2 = Client::new(&node2_cfg);

    let bucket_name = format!("multinode-sync-test-{}", Uuid::new_v4());
    let bucket = bucket_name.as_str();
    let key = "sample.txt";
    let content = "hello world\n你好世界\n";
    let crc32c =
        base64_simd::STANDARD.encode_to_string(crc32c::crc32c(content.as_bytes()).to_be_bytes());

    create_bucket(&client1, bucket).await?;
    {
        let body = ByteStream::from_static(content.as_bytes());
        client1
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(body)
            .metadata("meta", "pig")
            .checksum_crc32_c(crc32c.as_str())
            .send()
            .await?;
    }

    // share bucket
    let ticket = node1_handle.share_bucket(bucket.to_string()).await.unwrap();
    // import on other node
    let mut events = node2_handle.import_bucket(ticket).await.unwrap();
    // wait for sync
    tokio::time::timeout(Duration::from_secs(10), async {
        while let Some(Ok(ev)) = events.next().await {
            if matches!(ev, iroh_docs::engine::LiveEvent::SyncFinished(_)) {
                println!("match?");
                break;
            }
        }
    })
    .await
    .expect("Sync took too long");

    {
        let ans = client2
            .get_object()
            .bucket(bucket)
            .key(key)
            .checksum_mode(ChecksumMode::Enabled)
            .send()
            .await?;

        let content_length: usize = ans.content_length().unwrap().try_into().unwrap();
        let body = ans.body.collect().await?.into_bytes();
        let metadata = ans.metadata.unwrap();

        assert_eq!(content_length, content.len());
        assert_eq!(metadata.get("meta"), Some(&("pig".to_string())));
        assert_eq!(body.as_ref(), content.as_bytes());
    }

    {
        // delete_object(&client1, bucket, key).await?;
        delete_bucket(&client1, bucket).await?;
        delete_object(&client2, bucket, key).await?;
        delete_bucket(&client2, bucket).await?;
        fs::remove_dir_all(NODE1_FS_ROOT).unwrap();
        fs::remove_dir_all(NODE2_FS_ROOT2).unwrap();
    }

    Ok(())
}
