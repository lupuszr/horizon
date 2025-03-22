# 🌐 HorizonPush

> 🚧 **Experimental & Work in Progress**: HorizonPush is actively being developed. Expect breaking changes, incomplete features, and evolving APIs.

HorizonPush is a file-sharing and synchronization tool leveraging `iroh` for peer-to-peer transfers and multi-provider blob streaming. It provides an extensible WebAssembly (Wasm) API, integrates with S3 for storage, and includes a Tauri-based desktop application.

## ⭐ Features

- 📂 **Decentralized File Sharing**: Uses `iroh-blobs` for secure, verifiable blob transfers.
- 🔀 **Multi-Provider Blob Streaming**: Enables efficient file distribution by leveraging multiple sources.
- 🧩 **WebAssembly Plugin Support**: Extend functionality with third-party Wasm plugins.
- ☁️ **S3 Integration**: Acts as an S3-compatible storage proxy with local caching.
- 🖥️ **Tauri Desktop App**: Provides a cross-platform, native UI for managing transfers.

## 🧩 WebAssembly API

HorizonPush exposes a Wasm API, allowing developers to extend its functionality with custom plugins. The Wasm runtime has access to:

- 📜 `iroh-docs`: Multi-dimensional key-value document store.
- 📦 `iroh-blobs`: Secure blob transfers with BLAKE3 verification.
- ⚙️ `horizon-sdk`: Helper functions for interacting with HorizonPush internals.

## 📜 iroh Integration with S3

HorizonPush acts as an S3 proxy with intelligent caching:

- 🔄 **S3 API Compatibility**: Supports standard S3 operations (PUT, GET, DELETE).
- 💾 **Local-First Syncing**: Keeps frequently accessed files locally.
- 🗄️ **Transparent Caching**: Moves rarely used files to S3 storage.

## 🖥️ Tauri Desktop App

The HorizonPush desktop application provides a native UI for:

- 📂 Managing file transfers
- ⚙️ Configuring S3 settings
- 🧩 Installing and managing Wasm plugins

### ▶️ Running the Tauri App

```sh
cd crates/apps/horizon-push
npm install
npm run tauri dev
```

### ▶️ Running the Horizon Server

```sh
cargo run --bin horizon-push start-server
```

## 🤝 Contributing

Contributions are welcome! Submit issues and pull requests on GitHub.

