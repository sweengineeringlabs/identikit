# Identikit

**Digital facial composite creation platform for law enforcement and forensic applications.**

## Features

- Comprehensive facial feature library (hair, face shape, eyes, eyebrows, nose, mouth, chin, ears, accessories)
- Layer-based compositing with Fabric.js
- Real-time preview and adjustment controls (position, scale, rotation, opacity)
- Export to PNG and PDF formats
- Save/load composite files (.idkit format)
- Full undo/redo history
- Keyboard shortcuts for efficient workflow

## Quick Start

```bash
# Clone repository
git clone <repository-url>
cd identikit

# Install frontend dependencies
cd frontend && bun install

# Run development mode
bun run tauri dev
```

## Build

```bash
# Build for production
bun run tauri build
```

## Documentation

See [docs/overview.md](docs/overview.md) for complete documentation.

## Tech Stack

| Component | Technology |
|-----------|------------|
| Desktop Framework | Tauri 2.0 |
| Backend | Rust |
| Frontend | React + TypeScript |
| Build Tool | Vite |
| Styling | Tailwind CSS |
| Canvas | Fabric.js |
| State Management | Zustand |

## License

MIT License - see [LICENSE](LICENSE) for details.
