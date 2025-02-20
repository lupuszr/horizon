use std::path::Path;
use std::process::Command;

fn main() {
    // let out_dir = std::env::var("OUT_DIR").unwrap();
    // let wasm_path = Path::new(&out_dir).join("wasm_example.wasm");
    // // Shorter target directory path
    // let cargo_target_dir = format!("{}/wasm_target", out_dir); // Shorter path!

    // std::fs::create_dir_all(&cargo_target_dir).unwrap(); // Create the directory
    // println!("wtf {}\n\n\n", out_dir);
    // let status = Command::new("cargo")
    //     .args(&["build", "--target", "wasm32-unknown-unknown"])
    //     // .env("CARGO_TARGET_DIR", "/Users/lupus/projects/horizon/wasm_out") // Use the shorter path
    //     .current_dir(".")
    //     .status()
    //     .expect("Failed to build WASM");

    // if !status.success() {
    //     panic!("Rust WASM compilation failed");
    // }

    // println!("wth 2\n\n\n");

    // // Generate the wasm-bindgen glue code
    // let status = Command::new("wasm-bindgen")
    //     .args(&[
    //         "--target",
    //         "nodejs",
    //         "--out-dir",
    //         &out_dir,
    //         "--out-name",
    //         "wasm_example",
    //         wasm_path.to_str().unwrap(),
    //     ]) // Use nodejs target for general use
    //     .status()
    //     .expect("Failed to generate wasm-bindgen glue code");

    // if !status.success() {
    //     panic!("wasm-bindgen failed");
    // }

    // println!("cargo:rerun-if-changed=crates/wasm_plugins/rust_example/src/lib.rs");
}
