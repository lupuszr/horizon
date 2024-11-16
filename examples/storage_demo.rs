use horizon_storage::{DefaultStorage, Storage};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Create storage instance
    let storage = DefaultStorage::new().await?;

    // Example: Upload a file
    let file_path = PathBuf::from("example.mp4");
    let content_id = storage.upload_file(&file_path).await?;
    println!("Uploaded file with ID: {}", content_id.0);

    // Example: Download the file
    let download_path = PathBuf::from("downloaded_example.mp4");
    storage.download_to_file(&content_id, &download_path).await?;
    println!("Downloaded file to: {}", download_path.display());

    // Example: Check if content exists
    let exists = storage.exists(&content_id).await?;
    println!("Content exists: {}", exists);

    Ok(())
}
