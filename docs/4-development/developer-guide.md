# Developer Guide

**Audience**: Developers, Contributors

## WHAT: Development Setup

This guide covers setting up a development environment for Identikit.

## WHY: Prerequisites

Before starting, ensure you have the required tools installed.

## HOW: Setup Steps

### Prerequisites

| Tool | Version | Install Command |
|------|---------|-----------------|
| Rust | 1.75+ | [rustup.rs](https://rustup.rs) |
| Bun | 1.0+ | `curl -fsSL https://bun.sh/install \| bash` |
| Tauri CLI | 2.0+ | `cargo install tauri-cli` |

#### Linux Additional Dependencies

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

#### macOS Additional Dependencies

```bash
xcode-select --install
```

### Clone and Setup

```bash
# Clone repository
git clone <repository-url>
cd identikit

# Install frontend dependencies
cd frontend
bun install

# Return to root
cd ..
```

### Development Mode

```bash
# Start development server with hot reload
cd frontend
bun run tauri dev
```

This will:
1. Start Vite dev server on `http://localhost:5173`
2. Build the Rust backend
3. Launch the Tauri window

### Project Structure

```
identikit/
├── backend/           # Rust/Tauri backend
│   ├── src/
│   │   ├── main.rs    # Entry point
│   │   ├── commands/  # Tauri commands
│   │   ├── models/    # Data structures
│   │   └── services/  # Business logic
│   └── Cargo.toml
│
├── frontend/          # React frontend
│   ├── src/
│   │   ├── components/
│   │   ├── hooks/
│   │   ├── stores/
│   │   └── types/
│   ├── package.json
│   └── vite.config.ts
│
└── docs/              # Documentation
```

### Common Commands

```bash
# Frontend
cd frontend
bun run dev          # Start Vite dev server only
bun run build        # Build frontend
bun run lint         # Run ESLint
bun run typecheck    # Run TypeScript check

# Backend
cd backend
cargo check          # Check compilation
cargo test           # Run tests
cargo clippy         # Run linter
cargo fmt            # Format code

# Tauri
cd frontend
bun run tauri dev    # Development mode
bun run tauri build  # Production build
```

### Adding a New Tauri Command

1. Define the command in `backend/src/commands/`:

```rust
#[tauri::command]
pub fn my_command(param: String) -> Result<String, String> {
    Ok(format!("Received: {}", param))
}
```

2. Register in `backend/src/main.rs`:

```rust
.invoke_handler(tauri::generate_handler![
    commands::my_command,
    // ... other commands
])
```

3. Call from frontend:

```typescript
import { invoke } from '@tauri-apps/api/tauri';

const result = await invoke<string>('my_command', { param: 'hello' });
```

### Adding a New React Component

1. Create component file in appropriate directory
2. Export from index.ts barrel file
3. Add TypeScript types in `src/types/`
4. Update Zustand store if needed

### Testing

```bash
# Backend tests
cd backend
cargo test

# Frontend E2E tests
cd frontend
bun run test:e2e
```

## Troubleshooting

### Tauri build fails on Linux

Ensure all system dependencies are installed (see Prerequisites).

### Hot reload not working

Try restarting the dev server: `bun run tauri dev`

### TypeScript errors

Run `bun run typecheck` to see all errors.

## Related

- [Architecture](../3-design/architecture.md)
- [Contributing Guide](../../CONTRIBUTING.md)

---

**Last Updated**: 2025-01-01
