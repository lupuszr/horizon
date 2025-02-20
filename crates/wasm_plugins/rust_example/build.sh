#!/bin/bash

# Set variables for clarity and easy modification
package_name="rust_example"
target="wasm32-unknown-unknown"
out_dir="./pkg"  # Or your desired output directory
wasm_name="wasm_example" # Name of the wasm output file
# Set CARGO_TARGET_DIR if not already set.  This allows the user to override it.
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-./target}" # Defaults to ./target if CARGO_TARGET_DIR isn't set.
wasm_path="$CARGO_TARGET_DIR/$target/release/$package_name.wasm"
export RUSTFLAGS="--cfg tokio_unstable"

echo "pina"
echo $RUSTFLAGS
# Create the output directory if it doesn't exist
mkdir -p "$out_dir"

# Run cargo build with verbose output
cargo build -vv -p "$package_name" --target "$target" --release

# Check if cargo build was successful
# if [ $? -eq 0 ]; then
#   # Run wasm-bindgen
#   wasm-bindgen --target nodejs --out-dir "$out_dir" --out-name "$wasm_name" "$wasm_path"

#   # Check if wasm-bindgen was successful
#   if [ $? -eq 0 ]; then
#     echo "Build and wasm-bindgen complete successfully!"
#   else
#     echo "wasm-bindgen failed!"
#   fi
# else
#   echo "cargo build failed!"
# fi
