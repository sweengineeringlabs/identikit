# Layout C: Bottom Dock

This folder contains all UI designs for the **Bottom Dock** layout variant.

## Layout Overview

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  File  Edit  View  Help                                              [_][□][✕]          │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                         │
│  ┌────────────────────────────────────────────────────────┐  ┌──────────────────────┐  │
│  │                                                        │  │ Layers / Adjustments │  │
│  │                                                        │  │                      │  │
│  │                                                        │  │ [Tab content...]     │  │
│  │                   CANVAS AREA                          │  │                      │  │
│  │                   (Large)                              │  │                      │  │
│  │                                                        │  │                      │  │
│  │                                                        │  │                      │  │
│  └────────────────────────────────────────────────────────┘  └──────────────────────┘  │
│                                                                                         │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│  Feature Library (Docked Bottom)                                      [▲ Collapse]      │
│  ┌──────────────────────────────────────────────────────────────────────────────────┐   │
│  │ [🔍 Search...]  Hair | Eyes | Nose | Mouth | Eyebrows | Ears | Chin | Accessories│   │
│  │                                                                                   │   │
│  │ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐   │   │
│  │ │    │ │    │ │    │ │    │ │    │ │    │ │    │ │    │ │    │ │    │ │    │   │   │
│  │ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘   │   │
│  │                                                                              ► │   │
│  └──────────────────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Key Characteristics

- **Feature Library at Bottom**: Horizontal dock with category tabs, maximizes feature visibility
- **Large Canvas**: Canvas takes most of the vertical space
- **Minimal Right Panel**: Combined Layers/Adjustments in compact tabbed panel
- **Collapsible Dock**: Bottom dock can collapse to give full canvas space
- **Horizontal Scrolling**: Features scroll horizontally within categories

## Folder Structure

```
layout-c/
├── README.md                    # This file
├── pages/
│   ├── 00-welcome.md           # Welcome/no-composite view
│   ├── 01-workspace.md         # Main workspace
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
│   ├── toolbar.md              # Top toolbar
│   ├── feature-dock.md         # Bottom feature dock (unique to Layout C)
│   ├── canvas.md               # Main canvas area
│   ├── side-panel.md           # Right side panel (combined)
│   ├── adjustment-controls.md  # Adjustment sliders
│   └── compare-panel.md        # Compare mode controls
└── states/
    ├── loading.md              # Loading states
    ├── empty.md                # Empty states
    ├── error.md                # Error states
    └── onboarding.md           # First-time user experience
```

## Component Mapping

| Component | File Location |
|-----------|---------------|
| `AppLayout.tsx` | Uses bottom dock layout |
| `FeatureDock.tsx` | `components/feature-dock.md` |
| `SidePanel.tsx` | `components/side-panel.md` |
| `CompositeCanvas.tsx` | `components/canvas.md` |

## Comparison with Other Layouts

| Aspect | Layout C (Bottom Dock) | Layout 0 (Default) |
|--------|------------------------|-------------------|
| Feature Library | Horizontal dock at bottom | Vertical sidebar |
| Canvas Size | Large, full width | Medium, constrained |
| Category Navigation | Horizontal tabs | Accordion |
| Feature Display | Horizontal scroll | Grid scroll |
| Right Panel | Compact, tabbed | Full height |
| Dock Behavior | Collapsible | Fixed |

## When to Use This Layout

- Users with wide monitors (21:9 or dual monitors)
- Workflows requiring quick category switching
- Need for maximum canvas workspace
- Preference for horizontal feature browsing
- Touch-screen or tablet-optimized interfaces
