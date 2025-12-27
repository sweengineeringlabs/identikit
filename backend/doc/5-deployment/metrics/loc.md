# Backend Code Metrics

**Generated**: 2025-12-27
**Tool**: tokei
**Path**: `backend/src/`

## Summary

| Metric | Count |
|--------|------:|
| Files | 31 |
| Total Lines | 4,868 |
| Code | 3,568 |
| Comments | 577 |
| Blanks | 723 |

**Code density**: 73% code, 12% comments, 15% blanks

## By Language

| Language | Files | Lines | Code | Comments | Blanks |
|----------|------:|------:|-----:|---------:|-------:|
| Rust | 31 | 4,397 | 3,568 | 149 | 680 |
| Markdown (in Rust) | 31 | 471 | 0 | 428 | 43 |

## By Module

| Module | Files | Lines | Purpose |
|--------|------:|------:|---------|
| `ai/` | 10 | 1,723 | Face detection, embedding, alignment, comparison |
| `services/` | 4 | 1,217 | Core business logic |
| `commands/` | 7 | 1,027 | Tauri IPC command handlers |
| `models/` | 6 | 829 | Data structures |
| Root | 2 | 90 | Entry points |

## Largest Files

| File | Lines | Code |
|------|------:|-----:|
| `models/ai.rs` | 475 | ~380 |
| `services/image_processor.rs` | 450 | ~360 |
| `services/feature_library.rs` | 440 | ~350 |
| `ai/alignment/affine_aligner.rs` | 325 | ~260 |
| `services/pdf_generator.rs` | 318 | ~250 |
| `commands/ai.rs` | 278 | ~220 |
| `commands/export.rs` | 262 | ~210 |
| `ai/comparison/cosine.rs` | 251 | ~200 |
| `ai/state.rs` | 247 | ~200 |
| `ai/embedding/dlib_embedder.rs` | 232 | ~185 |

## Test Coverage

| Category | Tests |
|----------|------:|
| AI (alignment, comparison, detection, embedding, error, state, traits) | 21 |
| Models (ai) | 5 |
| Services (feature_library, image_processor, pdf_generator) | 9 |
| **Total** | **35** |

## Project Structure

```
backend/src/
├── main.rs
├── lib.rs
├── ai/
│   ├── mod.rs
│   ├── error.rs
│   ├── traits.rs
│   ├── state.rs
│   ├── alignment/
│   ├── comparison/
│   ├── detection/
│   └── embedding/
├── commands/
│   ├── mod.rs
│   ├── ai.rs
│   ├── composite.rs
│   ├── export.rs
│   ├── feature_library.rs
│   ├── file_io.rs
│   └── history.rs
├── models/
│   ├── mod.rs
│   ├── ai.rs
│   ├── composite.rs
│   ├── feature.rs
│   ├── layer.rs
│   └── transform.rs
└── services/
    ├── mod.rs
    ├── feature_library.rs
    ├── image_processor.rs
    └── pdf_generator.rs
```
