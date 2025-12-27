# Layout D: Split View Focus

This folder contains all UI designs for the **Split View Focus** layout variant.

## Layout Overview

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  File  Edit  View  Help                                              [_][□][✕]          │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│  [📄][💾][↩][↪] │ [Tools] │ Mode: [Edit●][Compare] │ Zoom │ Options                     │
├────────────────────────────────────────┬────────────────────────────────────────────────┤
│                                        │                                                │
│                                        │                                                │
│           FEATURE BROWSER              │              CANVAS / COMPARISON               │
│           (Scrollable grid)            │              (Large workspace)                 │
│                                        │                                                │
│  ┌────┐ ┌────┐ ┌────┐ ┌────┐          │                                                │
│  │    │ │    │ │    │ │    │          │                  👁️    👁️                       │
│  └────┘ └────┘ └────┘ └────┘          │                                                │
│  ┌────┐ ┌────┐ ┌────┐ ┌────┐          │                    👃                          │
│  │    │ │    │ │    │ │    │          │                                                │
│  └────┘ └────┘ └────┘ └────┘          │                    👄                          │
│                                        │                                                │
│  ────────────────────────────          │  ─────────────────────────────────────────────│
│                                        │                                                │
│  [Layers]                              │  [Adjustments]                                 │
│  [👁] Hair                             │  Position: X:[__] Y:[__]                       │
│  [👁] Eyes ●                           │  Scale: [━━━○━━] 100%                          │
│  [👁] Nose                             │  Rotation: [__]°                               │
│                                        │                                                │
└────────────────────────────────────────┴────────────────────────────────────────────────┘
```

## Key Characteristics

- **Vertical Split**: Screen divided into left (features) and right (canvas) panels
- **Feature Browser Left**: Full-height browsable feature grid with categories
- **Canvas Right**: Large canvas area with integrated adjustments below
- **Mode Toggle**: Quick switch between Edit and Compare modes
- **Integrated Panels**: Layers and Adjustments embedded in their respective sides

## Folder Structure

```
layout-d/
├── README.md                    # This file
├── pages/
│   ├── 00-welcome.md           # Welcome/no-composite view
│   ├── 01-workspace.md         # Main workspace (edit mode)
│   ├── 02-feature-browsing.md  # Feature selection
│   ├── 03-adjustments.md       # Layer adjustments
│   ├── 04-compare-mode.md      # Reference comparison
│   └── 05-print-preview.md     # Export preview
├── modals/
│   ├── new-composite.md        # New composite dialog
│   ├── export.md               # Export options
│   ├── case-info.md            # Case information form
│   ├── shortcuts.md            # Keyboard shortcuts
│   ├── feature-detail.md       # Feature detail view
│   ├── confirm-dialogs.md      # Confirmations
│   └── reference-upload.md     # Reference image upload
├── components/
│   ├── toolbar.md              # Top toolbar with mode toggle
│   ├── feature-browser.md      # Left panel feature browser
│   ├── canvas-panel.md         # Right panel with canvas
│   ├── layer-list.md           # Compact layer list
│   ├── adjustment-bar.md       # Bottom adjustment bar
│   └── compare-view.md         # Compare mode layout
└── states/
    ├── loading.md              # Loading states
    ├── empty.md                # Empty states
    ├── error.md                # Error states
    └── onboarding.md           # First-time user experience
```

## Component Mapping

| Component | File Location |
|-----------|---------------|
| `AppLayout.tsx` | Uses split view layout |
| `FeatureBrowser.tsx` | `components/feature-browser.md` |
| `CanvasPanel.tsx` | `components/canvas-panel.md` |
| `LayerList.tsx` | `components/layer-list.md` |
| `AdjustmentBar.tsx` | `components/adjustment-bar.md` |

## Comparison with Other Layouts

| Aspect | Layout D (Split View) | Layout 0 (Default) |
|--------|----------------------|-------------------|
| Primary Split | Vertical (left/right) | Vertical (3-column) |
| Feature Location | Left panel, full height | Left sidebar |
| Canvas Size | Large, right side | Center, constrained |
| Adjustments | Below canvas | Right sidebar |
| Layers | Below features | Right sidebar |
| Mode Toggle | Toolbar prominent | Menu/button |

## When to Use This Layout

- Focus on iterative feature selection and comparison
- Workflows alternating between editing and comparing
- Users who prefer clear separation of tasks
- Side-by-side reference comparison needs
- Presentation or demonstration contexts
