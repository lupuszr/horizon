use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;
use tempfile::{tempdir, NamedTempFile};
use tokio::time::sleep;

#[tokio::test]
async fn test_horizon_push_send_and_receive() {
    let ticket_file_path = "ticket.horizon";
    // Remove any leftover ticket file from previous runs
    if Path::new(ticket_file_path).exists() {
        fs::remove_file(ticket_file_path).unwrap();
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
            "--path",
            temp_file_path.to_str().unwrap(),
        ])
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn sender");

    // Poll for the ticket.horizon file
    let timeout = Duration::from_secs(30); // Maximum wait time
    let mut elapsed = Duration::ZERO;

    while !Path::new(ticket_file_path).exists() {
        sleep(Duration::from_millis(50)).await; // Check every 500ms
        elapsed += Duration::from_millis(50);
        if elapsed >= timeout {
            panic!("Timeout waiting for ticket.horizon to appear");
        }
    }

    let ticket: String = fs::read_to_string("ticket.horizon").unwrap();
    println!("ticket is:: \n {} \n", ticket);

    // Set up a temporary file path for receiving
    let receiver_folder = std::env::current_dir();
    let receiver_folder = format!("{}/receiver", receiver_folder.unwrap().to_str().unwrap()); //.with_file_name("received_file");
    println!("\n\n\n receive:: {:?}", receiver_folder);

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
            "--path",
            receiver_folder.as_str(),
        ])
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn receiver");

    // Wait for the receiver process to finish
    let status = receiver.wait().expect("Failed to wait on receiver");
    assert!(status.success(), "Receiver failed");

    // Verify the content of the received file
    // let received_content = fs::read(receiver_folder).expect("Failed to read received file");
    // println!("received contet:: {:?}", received_content);
    // assert_eq!(content, received_content.as_slice());

    // Kill both process
    sender.kill().unwrap();
    receiver.kill().unwrap();
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use horizon_push_cli::{
//         common::{CommonArgs, RelayModeOption},
//         error::AppError,
//         receive::HorizonPushReceive,
//         send::HorizonPushSend,
//     };
//     use tempfile::{tempdir, NamedTempFile};

//     use tokio::{fs, io::AsyncReadExt, sync::mpsc};

//     #[tokio::test]
//     async fn test_horizon_push_send_and_receive() -> Result<(), AppError> {
//         // Set up a temporary file to simulate sending a file
//         let temp_file = NamedTempFile::new().unwrap();
//         let temp_file_path = temp_file.path().to_path_buf();

//         // Write some content to the temporary file
//         let content = b"Hello, Horizon!";
//         fs::write(&temp_file_path, content).await.unwrap();

//         // Set up the channel for ticket communication
//         let (tx, mut rx) = mpsc::channel(1);

//         // Simulate sending the file
//         let send = HorizonPushSend {
//             path: temp_file_path.clone(),
//             common: CommonArgs {
//                 magic_ipv4_addr: None,
//                 magic_ipv6_addr: None,
//                 verbose: 1,
//                 relay: RelayModeOption::Default,
//             },
//         };

//         tokio::spawn(async move {
//             send.eval(tx).await.unwrap();
//         });

//         // Set up a temporary directory to receive the file
//         let temp_dir = tempdir().unwrap();
//         let received_file_path = temp_dir.path().join("received_file");

//         let ticket = rx.recv().await.unwrap();

//         // set up a channel to receive if download is finished
//         let (tx, mut rx) = mpsc::channel(1);
//         // Simulate receiving the file

//         // tokio::spawn(async move {
//         //     let temp_dir = tempdir().unwrap();
//         //     let received_file_path = temp_dir.path().join("received_file");
//         //     let receive = HorizonPushReceive {
//         //         path: received_file_path.clone(),
//         //         url: "".to_string(),
//         //         common: CommonArgs {
//         //             magic_ipv4_addr: None,
//         //             magic_ipv6_addr: None,
//         //             verbose: 0,
//         //             relay: RelayModeOption::Default,
//         //         },
//         //     }; // Adjust as needed
//         //     receive.eval(tx).await.unwrap();
//         // });

//         let receive = HorizonPushReceive {
//             path: received_file_path.clone(),
//             url: ticket,
//             common: CommonArgs {
//                 magic_ipv4_addr: None,
//                 magic_ipv6_addr: None,
//                 verbose: 0,
//                 relay: RelayModeOption::Default,
//             },
//         }; // Adjust as needed
//         receive.eval(tx).await.unwrap();
//         let finished = rx.recv().await.unwrap();

//         assert_eq!(finished, true);
//         println!("jel me jebes?");
//         // Read the received file and compare the content
//         let mut received_content = Vec::new();
//         let mut received_file = fs::File::open(received_file_path).await.unwrap();
//         received_file
//             .read_to_end(&mut received_content)
//             .await
//             .unwrap();

//         assert_eq!(content.to_vec(), received_content.to_vec());

//         Ok(())
//     }
// }

// use std::fs;
// use std::path::{Path, PathBuf};
// use std::process::{Command, Stdio};
// use std::time::Duration;
// use tempfile::tempdir;
// use tokio::time::sleep;

#[tokio::test]
async fn test_horizon_push_send_and_receive_folder() {
    let ticket_file_path = "ticket.horizon";
    // Remove any leftover ticket file from previous runs
    if Path::new(ticket_file_path).exists() {
        fs::remove_file(ticket_file_path).unwrap();
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
            "--path",
            folder_path.to_str().unwrap(),
        ])
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn sender");

    // Poll for the ticket.horizon file
    let timeout = Duration::from_secs(30); // Maximum wait time
    let mut elapsed = Duration::ZERO;

    while !Path::new(ticket_file_path).exists() {
        sleep(Duration::from_millis(50)).await;
        elapsed += Duration::from_millis(50);
        if elapsed >= timeout {
            panic!("Timeout waiting for ticket.horizon to appear");
        }
    }

    let ticket: String = fs::read_to_string(ticket_file_path).unwrap();
    println!("Ticket is: \n {} \n", ticket);

    // Set up a temporary folder for receiving
    let receiver_folder = std::env::current_dir().unwrap().join("receiver_folder");
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
