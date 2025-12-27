# Architecture

**Audience**: Developers, Technical Architects

## WHAT: System Architecture

Identikit follows a layered architecture with clear separation between the Tauri/Rust backend and the React/TypeScript frontend.

## WHY: Design Decisions

1. **Tauri over Electron** - Smaller binary size, better security, native performance
2. **Fabric.js** - Mature canvas library with object model and manipulation
3. **Zustand** - Lightweight state management, works well with React
4. **Rust backend** - Type-safe, performant file I/O and image processing
5. **ONNX Runtime** - Local AI inference for face detection and recognition

## HOW: Architecture Layers

### Layer Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                      Presentation Layer                      │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────────┐│
│  │  Toolbar    │ │  Sidebar    │ │     Main Canvas         ││
│  └─────────────┘ └─────────────┘ └─────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│                      Component Layer                         │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐            │
│  │FeatureBrowser│ │CompositeCanvas│ │AdjustmentPanel│       │
│  └─────────────┘ └─────────────┘ └─────────────┘            │
├─────────────────────────────────────────────────────────────┤
│                      State Layer (Zustand)                   │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐            │
│  │compositeStore│ │historyStore │ │  uiStore    │           │
│  └─────────────┘ └─────────────┘ └─────────────┘            │
├─────────────────────────────────────────────────────────────┤
│                      Service Layer                           │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                   Tauri API Bridge                       ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│                      Backend Layer (Rust)                    │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐            │
│  │  Commands   │ │   Models    │ │  Services   │            │
│  └─────────────┘ └─────────────┘ └─────────────┘            │
├─────────────────────────────────────────────────────────────┤
│                      AI Layer (ONNX Runtime)                 │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐            │
│  │Face Detector│ │  Aligner    │ │  Embedder   │            │
│  │  (SCRFD)    │ │ (Landmarks) │ │ (ArcFace)   │            │
│  └─────────────┘ └─────────────┘ └─────────────┘            │
└─────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility |
|-----------|----------------|
| CompositeCanvas | Fabric.js rendering, object manipulation |
| FeatureBrowser | Category navigation, feature selection |
| AdjustmentPanel | Transform controls for selected layer |
| LayerManager | Layer ordering, visibility, locking |
| MainToolbar | File operations, undo/redo, export |

### State Management

```typescript
// compositeStore - Central composite state
{
  composite: Composite | null,
  selectedLayerId: string | null,
  isModified: boolean,
  // Actions
  addLayer, removeLayer, updateLayerTransform, ...
}

// historyStore - Undo/redo management
{
  undoStack: HistoryAction[],
  redoStack: HistoryAction[],
  // Actions
  undo, redo, pushAction, ...
}

// uiStore - UI state
{
  zoom: number,
  panelVisibility: { layers: boolean, adjustment: boolean },
  // Actions
  setZoom, togglePanel, ...
}
```

### Backend Commands

| Command | Purpose | Parameters |
|---------|---------|------------|
| `create_composite` | New composite | name |
| `add_layer` | Add feature layer | featureId, transform |
| `update_layer_transform` | Modify transform | layerId, transform |
| `remove_layer` | Delete layer | layerId |
| `save_composite` | Save to file | path |
| `load_composite` | Load from file | path |
| `export_png` | Export image | path, width, height |
| `export_pdf` | Export document | path, includeMetadata |
| `detect_faces` | Find faces in image | image_path |
| `compare_faces` | Compare two photos | image_a, image_b |
| `compare_to_composite` | Compare composite to reference | composite_png, reference_path |

## Security Considerations

- Tauri's strict CSP prevents XSS
- File access restricted via Tauri's fs plugin
- No network access by default
- Input validation on all commands

## Related

- [File Format Specification](file-format-spec.md)
- [AI Face Recognition](ai/face-recognition.md)
- [Toolchain](toolchain.md)
- [ADR Index](adr/README.md)

---

**Last Updated**: 2025-12-27
