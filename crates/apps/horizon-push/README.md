# HorizonPush
## P2P File Sharing Desktop Application

HorizonPush is a desktop application built with Tauri that enables instant P2P file sharing with professional features and S3 compatibility.

## Features

- 📤 Instant P2P file sharing
- 💾 S3 compatible storage integration
- 🔌 Plugin system for extensibility
- 🖥️ Native performance
- 🌐 Cross-platform support
- 🔒 Secure file transfer
- 📱 Mobile-friendly sharing links

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/horizonpush

# Navigate to the project
cd horizonpush

# Install dependencies
npm install # or yarn

# Run in development
- For Desktop development, run:
  npm run tauri dev

- For Android development, run:
  npm run tauri android dev

- For iOS development, run:
  npm run tauri ios dev
```

## System Requirements

- Windows 10/11, macOS 10.15+, or Linux
- 4GB RAM minimum
- Internet connection for P2P functionality

## Building

```bash
# Build for production
npm run tauri build
```

## Development

### Project Structure
```
horizonpush/
├── src/
│   ├── components/     # React components
│   ├── lib/           # Core functionality
│   ├── plugins/       # Plugin system
│   └── styles/        # CSS/styling
├── src-tauri/         # Rust backend code
│   ├── src/           
│   └── Cargo.toml     
└── package.json
```

### Technology Stack
- Tauri (Rust)
- React
- TypeScript
- Iroh (P2P)

## Plugin Development

Create plugins to extend functionality:

```typescript
// Example plugin
export class MyPlugin implements Plugin {
  name = 'my-plugin';
  version = '1.0.0';

  async init() {
    // Plugin initialization
  }

  // Plugin implementation
}
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- Documentation: [docs.horizonpush.com](https://docs.horizonpush.com)
- Issues: GitHub Issues
- Discord: [Join our community](https://discord.gg/horizonpush)

## Acknowledgments

- [Tauri](https://tauri.app)
- [Iroh](https://github.com/n0-computer/iroh)
- All contributors and community members


## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


Template created! To get started run:
  cd horizon-push
  npm install
  npm run tauri android init
  npm run tauri ios init

For Desktop development, run:
  npm run tauri dev

For Android development, run:
  npm run tauri android dev

For iOS development, run:
  npm run tauri ios dev
