# ADR-001: Backend Framework Selection

## Status

**Accepted**

## Context

Identikit requires a Rust backend for:
- File I/O operations (.idkit format)
- Image processing and export (PNG, PDF)
- AI inference (face detection, embeddings)
- Desktop integration via Tauri

We need to decide on the backend architecture approach.

## Options Considered

### Option 1: Tauri Commands Only

Use Tauri's built-in command system for all backend functionality.

```rust
#[tauri::command]
fn save_composite(path: String, data: Composite) -> Result<(), String> {
    // Direct implementation
}
```

**Pros:**
- Simplest architecture
- Direct IPC between frontend and backend
- No additional dependencies
- Tight Tauri integration

**Cons:**
- No HTTP API for external integrations
- Commands coupled to Tauri runtime

### Option 2: Axum Embedded

Embed Axum web server alongside Tauri for HTTP API.

```rust
// Tauri + Axum running together
let app = Router::new()
    .route("/api/composite", post(save_composite));
```

**Pros:**
- HTTP API for external tools
- REST/GraphQL support
- Could run headless

**Cons:**
- Added complexity
- Two communication channels
- Port management

### Option 3: Actix Web Embedded

Similar to Axum but using Actix Web framework.

**Pros:**
- Highest performance benchmarks
- Actor model for concurrency

**Cons:**
- Steeper learning curve
- More complex than needed

### Option 4: Loco Framework

Full-stack Rails-like framework.

**Pros:**
- Batteries included (ORM, migrations, CLI)
- Convention over configuration

**Cons:**
- Overkill for desktop app
- Opinionated structure conflicts with Tauri

## Decision

**Option 1: Tauri Commands Only**

For the initial implementation, use Tauri's native command system exclusively.

Rationale:
1. Desktop-first application - no HTTP API needed initially
2. Simplest architecture reduces bugs and maintenance
3. Tauri commands provide type-safe IPC
4. Can add Axum later if HTTP API becomes necessary

## Implementation

```
backend/src/
├── main.rs              # Tauri entry, command registration
├── commands/            # Tauri commands (IPC handlers)
│   ├── mod.rs
│   ├── composite.rs     # CRUD operations
│   ├── export.rs        # PNG/PDF export
│   ├── file_io.rs       # Save/load .idkit
│   └── ai.rs            # Face detection/comparison
├── models/              # Data structures
├── services/            # Business logic
└── ai/                  # ONNX inference (SPI providers)
```

## Consequences

### Positive
- Simple, maintainable codebase
- Fast development iteration
- No port conflicts or networking complexity
- Single communication channel

### Negative
- No REST API for external tools (can add later)
- Cannot run backend headless without Tauri

### Mitigations
- If HTTP API needed later, Axum can be added alongside Tauri commands
- Core logic in `services/` layer enables reuse

## References

- [Tauri Commands](https://tauri.app/v2/develop/calling-rust/)
- [Axum](https://github.com/tokio-rs/axum)
- [Actix Web](https://actix.rs/)
- [Loco](https://loco.rs/)

---

**Date**: 2025-12-27
**Author**: Engineering Team
