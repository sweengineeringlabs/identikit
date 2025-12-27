# Identikit Documentation

**Audience**: All stakeholders (developers, designers, end-users, law enforcement)

## WHAT: Identikit Platform

Identikit is a digital facial composite creation platform designed for law enforcement and forensic applications. It enables users to create facial composites by selecting and combining facial features from a comprehensive library.

Key capabilities:
- **Feature Library** - Categorized collection of facial features (hair, eyes, nose, etc.)
- **Layer Compositing** - Combine multiple features with precise positioning
- **Real-time Adjustment** - Position, scale, rotate, and adjust opacity
- **File Management** - Save, load, and export composites
- **History System** - Full undo/redo support

## WHY: Problems Solved

1. **Traditional Methods** - Physical overlays are cumbersome and limited
2. **Witness Memory** - Digital tools allow quick iteration to match memory
3. **Standardization** - Consistent feature library across investigations
4. **Distribution** - Easy export and sharing of composites

## Quick Navigation

| Section | Description | Audience |
|---------|-------------|----------|
| [Architecture](3-design/architecture.md) | System design and patterns | Developers |
| [File Format Spec](3-design/file-format-spec.md) | .idkit format specification | Developers |
| [Developer Guide](4-development/developer-guide.md) | Development setup and guides | Developers |
| [Deployment](6-deployment/overview.md) | Build and deployment instructions | DevOps |

## Module Overview

### Backend (Rust/Tauri)

| Module | Purpose | Location |
|--------|---------|----------|
| Commands | Tauri IPC handlers | `backend/src/commands/` |
| Models | Data structures | `backend/src/models/` |
| Services | PDF/image processing | `backend/src/services/` |

### Frontend (React/TypeScript)

| Component Area | Purpose | Location |
|----------------|---------|----------|
| Canvas | Fabric.js compositing | `frontend/src/components/canvas/` |
| Feature Library | Feature browsing | `frontend/src/components/feature-library/` |
| Adjustment | Transform controls | `frontend/src/components/adjustment/` |
| Toolbar | Actions and menus | `frontend/src/components/toolbar/` |

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Tauri Application                        │
├─────────────────────────────┬───────────────────────────────┤
│      Frontend (React)       │       Backend (Rust)          │
│                             │                               │
│  ┌─────────────────────┐    │    ┌─────────────────────┐    │
│  │   Fabric.js Canvas  │◄───┼───►│   Tauri Commands    │    │
│  └─────────────────────┘    │    └─────────────────────┘    │
│           │                 │             │                 │
│  ┌─────────────────────┐    │    ┌─────────────────────┐    │
│  │   Zustand Stores    │    │    │   File I/O          │    │
│  └─────────────────────┘    │    └─────────────────────┘    │
│           │                 │             │                 │
│  ┌─────────────────────┐    │    ┌─────────────────────┐    │
│  │   React Components  │    │    │   Export Services   │    │
│  └─────────────────────┘    │    └─────────────────────┘    │
└─────────────────────────────┴───────────────────────────────┘
```

## Data Flow

```
User Action → React Component → Zustand Store → Fabric.js Canvas
                                      │
                                      ▼
                               Tauri Command
                                      │
                                      ▼
                               Rust Backend
                                      │
                          ┌───────────┴───────────┐
                          ▼                       ▼
                    File System              Export Service
                   (.idkit files)           (PNG, PDF)
```

## Getting Started

1. **Users**: Download from releases, install, and start creating composites
2. **Developers**: See [Developer Guide](4-development/developer-guide.md)
3. **Contributors**: See [CONTRIBUTING.md](../CONTRIBUTING.md)

## Related Documentation

- [Architecture Decisions](3-design/adr/README.md)
- [Feature Backlog](backlog.md)

---

**Last Updated**: 2025-01-01
**Version**: 0.1.0
