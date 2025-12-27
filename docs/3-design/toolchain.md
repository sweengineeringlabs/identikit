# Toolchain

**Audience**: Developers, Technical Decision Makers

## Overview

| Layer | Technology | Version |
|-------|------------|---------|
| Desktop Framework | Tauri | 2.0 |
| Backend Language | Rust | 1.75+ |
| Frontend Framework | React | 18.x |
| Language | TypeScript | 5.x |
| Bundler | Vite | 5.x |
| Package Manager | Bun | 1.x |
| Canvas Library | Fabric.js | 6.x |
| State Management | Zustand | 4.x |
| Styling | Tailwind CSS | 3.x |

---

## Desktop Framework: Tauri 2.0

**Why Tauri over Electron:**

| Aspect | Tauri | Electron |
|--------|-------|----------|
| Binary size | ~3-10 MB | ~150+ MB |
| Memory usage | Lower | Higher |
| Security | Strict CSP, sandboxed | Less restrictive |
| Backend | Rust (native) | Node.js |
| Startup time | Faster | Slower |

**Trade-offs:**
- Smaller ecosystem than Electron
- Relies on system WebView (WebKit on Linux/macOS, WebView2 on Windows)
- Rust learning curve for backend development

---

## Backend: Rust

**Why Rust:**
- Memory safety without garbage collection
- Native performance for image processing
- Strong type system catches errors at compile time
- Excellent for file I/O operations
- First-class Tauri support

**Used for:**
- File operations (save/load .idkit files)
- Image processing (PNG export)
- PDF generation
- Undo/redo history management

**Key crates:**
| Crate | Purpose |
|-------|---------|
| `tauri` | Desktop framework |
| `serde` | Serialization/deserialization |
| `image` | Image processing |
| `resvg` | SVG rendering |
| `printpdf` | PDF generation |
| `uuid` | Unique identifiers |
| `chrono` | Date/time handling |

---

## Frontend: React + TypeScript

**Why React:**
- Component-based architecture
- Large ecosystem and community
- Excellent developer tooling
- Good Fabric.js integration examples

**Why TypeScript:**
- Type safety across frontend/backend boundary
- Better IDE support and refactoring
- Self-documenting code
- Catches errors before runtime

---

## Bundler: Vite

**Why Vite over Webpack:**

| Aspect | Vite | Webpack |
|--------|------|---------|
| Dev server startup | Instant (ESM) | Slow (bundling) |
| HMR speed | Fast | Slower |
| Configuration | Minimal | Complex |
| Build tool | Rollup | Webpack |

**Trade-offs:**
- Newer, smaller ecosystem
- Some plugins still Webpack-only

---

## Package Manager: Bun

**Why Bun over npm/yarn/pnpm:**

| Aspect | Bun | npm | pnpm |
|--------|-----|-----|------|
| Install speed | Fastest | Slowest | Fast |
| Disk usage | Low | High | Lowest |
| Compatibility | Good | Best | Good |
| Built-in bundler | Yes | No | No |

**Trade-offs:**
- Newer, less battle-tested
- Some edge cases with node compatibility
- Smaller community

---

## Canvas: Fabric.js

**Why Fabric.js:**
- Object model for canvas elements
- Built-in selection and manipulation
- Serialization/deserialization support
- Active maintenance
- Good documentation

**Alternatives considered:**

| Library | Why not chosen |
|---------|----------------|
| Konva.js | Less mature object model |
| Paper.js | More vector-focused, less object manipulation |
| PixiJS | Gaming-focused, overkill for this use case |
| Raw Canvas API | Too low-level, would need to build object model |

**Used for:**
- Rendering facial feature layers
- Object selection and manipulation
- Transform controls (scale, rotate, move)
- Layer compositing

---

## State Management: Zustand

**Why Zustand over Redux/MobX:**

| Aspect | Zustand | Redux | MobX |
|--------|---------|-------|------|
| Boilerplate | Minimal | Heavy | Medium |
| Bundle size | ~1KB | ~7KB | ~16KB |
| Learning curve | Easy | Steep | Medium |
| DevTools | Yes | Yes | Yes |
| TypeScript | Excellent | Good | Good |

**Store structure:**
```
stores/
├── compositeStore.ts   # Composite state, layers, selection
├── historyStore.ts     # Undo/redo stacks
├── featureLibraryStore.ts  # Feature categories and assets
└── uiStore.ts          # UI state (zoom, panels, mode)
```

---

## Styling: Tailwind CSS

**Why Tailwind:**
- Utility-first approach
- No context switching between files
- Consistent design system
- PurgeCSS for small production builds
- Good IDE support with IntelliSense

**Trade-offs:**
- Verbose class names in JSX
- Learning utility class names
- Less semantic HTML

---

## Development Tools

| Tool | Purpose |
|------|---------|
| ESLint | JavaScript/TypeScript linting |
| Prettier | Code formatting |
| Clippy | Rust linting |
| rustfmt | Rust formatting |
| Playwright | E2E testing |
| Cargo test | Rust unit tests |

---

## Version Requirements

```bash
# Minimum versions
rust >= 1.75.0
bun >= 1.0.0
node >= 18.0.0  # For compatibility tools

# Check versions
rustc --version
bun --version
```

---

## Platform Support

| Platform | WebView | Status |
|----------|---------|--------|
| Linux | WebKitGTK | Supported |
| macOS | WebKit | Supported |
| Windows | WebView2 | Supported |

---

## Related

- [Architecture](architecture.md)
- [Developer Guide](../4-development/developer-guide.md)
- [ADR Index](adr/README.md)

---

**Last Updated**: 2025-12-27
