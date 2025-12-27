# Layout B: Floating Panels

**Style**: Draggable, floating panels over full-screen canvas
**Best For**: Power users, multi-monitor setups, maximum canvas visibility

## Layout Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  Toolbar                                                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌─────────────┐                                                           │
│   │ Features    │                                                           │
│   │ [Draggable] │                                                           │
│   └─────────────┘                                                           │
│                                                                             │
│                    [Full Screen Canvas]                                     │
│                                                          ┌────────────────┐ │
│                                                          │ Adjustments    │ │
│                                                          │ [Draggable]    │ │
│                                                          └────────────────┘ │
│                                                                             │
│                                                          ┌────────────────┐ │
│                                                          │ Layers         │ │
│                                                          │ [Draggable]    │ │
│                                                          └────────────────┘ │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Key Differences from Default

| Aspect | Default (Layout 0) | Layout B |
|--------|-------------------|----------|
| Feature Library | Docked left | Floating, movable |
| Canvas | Center column | Full screen |
| Adjustments | Docked right | Floating, movable |
| Layers | Docked right | Floating, movable |
| Panels | Fixed position | User-positioned |

## Directory Structure

```
layout-b/
├── README.md
├── pages/
│   ├── 00-welcome.md
│   ├── 01-editor.md
│   ├── 02-compare.md
│   ├── 03-settings.md
│   ├── 04-help.md
│   └── 05-print-preview.md
├── modals/
│   ├── new-composite.md
│   ├── export.md
│   ├── case-info.md
│   ├── shortcuts.md
│   ├── feature-detail.md
│   ├── confirm-dialogs.md
│   └── reference-upload.md
├── components/
│   ├── toolbar.md
│   ├── floating-panel.md
│   ├── canvas.md
│   ├── layer-panel.md
│   ├── adjustment-panel.md
│   └── compare-panel.md
└── states/
    ├── loading.md
    ├── empty.md
    ├── error.md
    └── onboarding.md
```

## Panel Behavior

- **Draggable**: Click title bar to move
- **Resizable**: Drag edges/corners
- **Collapsible**: Double-click title to collapse
- **Dockable**: Snap to screen edges
- **Stackable**: Panels can overlap
- **Minimizable**: Reduce to toolbar icon

## When to Use This Layout

- Users who want maximum canvas space
- Multi-monitor setups (panels on secondary screen)
- Workflows that need frequent panel repositioning
- Users who prefer customizable workspace
