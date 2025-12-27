# Layout A: Horizontal Split

**Style**: Feature browser on top, canvas and tools below
**Best For**: Wide monitors, landscape orientations, focus on feature browsing

## Layout Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  Toolbar                                                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│  Feature Library (Horizontal Scrolling)                                     │
│  [Hair ▾] [Face ▾] [Eyes ▾] [Brows ▾] [Nose ▾] [Mouth ▾] [Ears ▾] [Acc ▾]  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                              │                              │
│                                              │   Adjustments                │
│              Canvas                          │   + Layers                   │
│                                              │                              │
│                                              │                              │
└──────────────────────────────────────────────┴──────────────────────────────┘
```

## Key Differences from Default

| Aspect | Default (Layout 0) | Layout A |
|--------|-------------------|----------|
| Feature Library | Left sidebar, vertical | Top bar, horizontal scroll |
| Canvas | Center | Bottom-left, larger |
| Adjustments | Right panel | Bottom-right |
| Layers | Right panel (below adj) | Bottom-right (tabbed) |

## Directory Structure

```
layout-a/
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
│   ├── feature-bar.md
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

## When to Use This Layout

- Ultra-wide monitors (21:9 or wider)
- Users who frequently browse features
- Workflows that prioritize feature selection
- Tablet landscape mode
