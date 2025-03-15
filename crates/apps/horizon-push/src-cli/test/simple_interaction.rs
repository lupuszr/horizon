use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;
use tempfile::{tempdir, NamedTempFile};
use tokio::time::sleep;

#[tokio::test]
#[ignore]
async fn test_horizon_push_send_and_receive() {
    let mut receiver_base_path = dirs_next::home_dir().unwrap();
    receiver_base_path.push(".horizon-push-receiver-simplefile");

    let mut sender_base_path = dirs_next::home_dir().unwrap();
    sender_base_path.push(".horizon-push-sender-simplefile");

    let mut ticket_path = sender_base_path.clone();
    ticket_path.push("ticket.horizon");
    // Remove any leftover ticket file from previous runs
    if ticket_path.exists() {
        fs::remove_file(ticket_path.clone()).unwrap();
    }
    // Set up a temporary file path for receiving
    let receiver_file = PathBuf::from("/tmp/horizontext");
    if receiver_file.exists() {
        fs::remove_file(receiver_file.clone()).unwrap();
    }

    // Create a temporary file to simulate sending a file
    let temp_file = NamedTempFile::new().unwrap();
    let temp_file_path = temp_file.path();
    let content = b"Hello, Horizon!";
    println!("temp file path:: {:?}", temp_file_path);
    fs::write(temp_file_path, content).unwrap();

    // Spawn the sender process
    let mut sender = Command::new("cargo")
        .env("RUST_BACKTRACE", "1")
        // .env("RUSTFLAGS", "-Awarnings")
        .args([
            "run",
            "-p",
            "horizon-push-cli",
            "send",
            "--base-path",
            sender_base_path.to_str().unwrap(),
            "--path",
            temp_file_path.to_str().unwrap(),
        ])
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn sender");

    // Poll for the ticket.horizon file
    let timeout = Duration::from_secs(30); // Maximum wait time
    let mut elapsed = Duration::ZERO;

    while !ticket_path.exists() {
        sleep(Duration::from_millis(50)).await; // Check every 500ms
        elapsed += Duration::from_millis(50);
        if elapsed >= timeout {
            panic!("Timeout waiting for ticket.horizon to appear");
        }
    }

    let ticket: String = fs::read_to_string(ticket_path).unwrap();
    println!("ticket is:: \n {} \n", ticket);

    // Spawn the receiver process
    let mut receiver = Command::new("cargo")
        .env("RUST_BACKTRACE", "1")
        // .env("RUSTFLAGS", "-Awarnings")
        .args([
            "run",
            "-p",
            "horizon-push-cli",
            "receive",
            "--url",
            &ticket.trim(),
            "--base-path",
            receiver_base_path.to_str().unwrap(),
            "--path",
            receiver_file.clone().to_str().unwrap(),
        ])
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn receiver");

    // Wait for the receiver process to finish
    let status = receiver.wait().expect("Failed to wait on receiver");
    assert!(status.success(), "Receiver failed");

    if receiver_file.exists() {
        fs::remove_file(receiver_file).unwrap();
    }

    // Verify the content of the received file
    // let received_content = fs::read(receiver_folder).expect("Failed to read received file");
    // println!("received contet:: {:?}", received_content);
    // assert_eq!(content, received_content.as_slice());

    // Kill both process
    sender.kill().unwrap();
    receiver.kill().unwrap();
}

#[tokio::test]
#[ignore]
async fn test_horizon_push_send_and_receive_folder() {
    let mut receiver_base_path = dirs_next::home_dir().unwrap();
    receiver_base_path.push(".horizon-push-receiver-multifile");

    let mut sender_base_path = dirs_next::home_dir().unwrap();
    sender_base_path.push(".horizon-push-sender-multifile");

    let mut ticket_path = sender_base_path.clone();
    ticket_path.push("ticket.horizon");
    // Remove any leftover ticket file from previous runs
    if ticket_path.exists() {
        fs::remove_file(ticket_path.clone()).unwrap();
    }

    // Remove any leftover from tmp folder from previous test
    let receiver_folder = PathBuf::from("/tmp/receiver_folder");
    if receiver_folder.exists() {
        fs::remove_dir_all(receiver_folder.clone()).unwrap();
    }

    // Create a temporary folder and populate it with files
    let temp_dir = tempdir().unwrap();
    let folder_path = temp_dir.path();
    let file_contents = vec![
        ("file1.txt", b"File 1 content"),
        ("file2.txt", b"File 2 content"),
        ("file3.txt", b"File 3 content"),
    ];

    for (file_name, content) in &file_contents {
        let file_path = folder_path.join(file_name);
        fs::write(&file_path, content).unwrap();
        println!("Created file: {:?}", file_path);
    }

    // Spawn the sender process with the folder
    let mut sender = Command::new("cargo")
        .env("RUST_BACKTRACE", "1")
        .args([
            "run",
            "-p",
            "horizon-push-cli",
            "send",
            "--base-path",
            sender_base_path.to_str().unwrap(),
            "--path",
            folder_path.to_str().unwrap(),
        ])
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn sender");

    // Poll for the ticket.horizon file
    let timeout = Duration::from_secs(30); // Maximum wait time
    let mut elapsed = Duration::ZERO;

    while !ticket_path.exists() {
        sleep(Duration::from_millis(50)).await;
        elapsed += Duration::from_millis(50);
        if elapsed >= timeout {
            panic!("Timeout waiting for ticket.horizon to appear");
        }
    }

    let ticket: String = fs::read_to_string(ticket_path).unwrap();
    println!("Ticket is: \n {} \n", ticket);

    // Set up a temporary folder for receiving
    if !receiver_folder.exists() {
        fs::create_dir(&receiver_folder).unwrap();
    }

    // Spawn the receiver process
    let mut receiver = Command::new("cargo")
        .env("RUST_BACKTRACE", "1")
        .args([
            "run",
            "-p",
            "horizon-push-cli",
            "receive",
            "--url",
            &ticket.trim(),
            "--base-path",
            receiver_base_path.to_str().unwrap(),
            "--path",
            receiver_folder.to_str().unwrap(),
        ])
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn receiver");

    // Wait for the receiver process to finish
    let status = receiver.wait().expect("Failed to wait on receiver");
    assert!(status.success(), "Receiver failed");

    // Verify the content of each received file
    for (file_name, expected_content) in &file_contents {
        let received_file_path = receiver_folder.join(file_name);
        println!("received_file_path {:?}", received_file_path);
        let received_content = fs::read(&received_file_path).expect("Failed to read received file");
        println!(
            "Received content for {}: {:?}",
            file_name,
            String::from_utf8_lossy(&received_content)
        );
        assert_eq!(
            expected_content.to_vec(),
            received_content.as_slice().to_vec()
        );
    }

    // Kill both processes
    sender.kill().unwrap();
    receiver.kill().unwrap();
}
