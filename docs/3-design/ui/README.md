# UI Screen Designs

**Audience**: Designers, Developers, Product Managers
**Status**: Complete

## Layout System

Identikit supports 5 different UI layouts to accommodate different workflows and user preferences. Each layout is fully documented with 23 design files covering pages, modals, components, and states.

| Layout | Description | Best For |
|--------|-------------|----------|
| **Layout 0** (Default) | Three-column with sidebar, canvas, right panel | Standard desktop workflow |
| **Layout A** | Horizontal split with feature browser on top | Widescreen monitors |
| **Layout B** | Floating draggable panels | Power users, multi-monitor |
| **Layout C** | Bottom dock with category tabs | Tablet/touch-friendly |
| **Layout D** | Vertical split (features left, canvas right) | Focus on comparison work |

## Directory Structure

```
ui/
├── README.md              # This file (layout overview)
│
├── layout-0/              # Default Three-Column Layout (23 files)
│   ├── README.md          # Layout overview
│   ├── pages/             # Full-page screens
│   ├── modals/            # Dialog/overlay screens
│   ├── components/        # Reusable UI components
│   └── states/            # UI states
│
├── layout-a/              # Horizontal Split Layout (23 files)
│   ├── README.md          # Layout overview
│   ├── pages/             # Full-page screens
│   ├── modals/            # Dialog/overlay screens
│   ├── components/        # Top/bottom split components
│   └── states/            # UI states
│
├── layout-b/              # Floating Panels Layout (23 files)
│   ├── README.md          # Layout overview
│   ├── pages/             # Full-page screens
│   ├── modals/            # Dialog/overlay screens
│   ├── components/        # Draggable floating panels
│   └── states/            # UI states
│
├── layout-c/              # Bottom Dock Layout (23 files)
│   ├── README.md          # Layout overview
│   ├── pages/             # Full-page screens
│   ├── modals/            # Dialog/overlay screens
│   ├── components/        # Horizontal dock components
│   └── states/            # UI states
│
└── layout-d/              # Split View Focus Layout (23 files)
    ├── README.md          # Layout overview
    ├── pages/             # Full-page screens
    ├── modals/            # Dialog/overlay screens
    ├── components/        # Vertical split components
    └── states/            # UI states
```

**Total: 115 UI design documents (23 per layout x 5 layouts)**

## Layout Previews

### Layout 0: Default Three-Column
```
┌──────────────────────────────────────────────────────────────────────────────┐
│  Toolbar                                                                     │
├────────────────┬─────────────────────────────────┬───────────────────────────┤
│                │                                 │                           │
│   SIDEBAR      │          CANVAS                 │     RIGHT PANEL           │
│   Feature      │                                 │     Adjustments           │
│   Browser      │                                 │     + Layers              │
│                │                                 │                           │
└────────────────┴─────────────────────────────────┴───────────────────────────┘
```

### Layout A: Horizontal Split
```
┌──────────────────────────────────────────────────────────────────────────────┐
│  Toolbar                                                                     │
├──────────────────────────────────────────────────────────────────────────────┤
│  FEATURE BROWSER (scrollable horizontal strip with categories)               │
├───────────────────────────────────────────────────┬──────────────────────────┤
│                                                   │                          │
│              CANVAS                               │    Layers + Adjustments  │
│              (Main workspace)                     │                          │
│                                                   │                          │
└───────────────────────────────────────────────────┴──────────────────────────┘
```

### Layout B: Floating Panels
```
┌──────────────────────────────────────────────────────────────────────────────┐
│  Toolbar                                                                     │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│   ┌─────────────────┐              CANVAS              ┌─────────────────┐   │
│   │ Features    [_] │              (Full workspace)    │ Adjustments [_] │   │
│   │ [Grid...]       │                                  │ [Controls...]   │   │
│   └─────────────────┘                                  └─────────────────┘   │
│                                 ┌─────────────────┐                          │
│                                 │ Layers      [_] │                          │
│                                 │ [List...]       │                          │
│                                 └─────────────────┘                          │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Layout C: Bottom Dock
```
┌──────────────────────────────────────────────────────────────────────────────┐
│  Toolbar                                                                     │
├──────────────────────────────────────────────────┬───────────────────────────┤
│                                                  │                           │
│                  CANVAS                          │   Side Panel              │
│                  (Large workspace)               │   Layers + Adjustments    │
│                                                  │                           │
├──────────────────────────────────────────────────┴───────────────────────────┤
│  [Hair] [Eyes] [Nose] [Mouth] ...   FEATURE DOCK (horizontal tabs + grid)    │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Layout D: Split View Focus
```
┌──────────────────────────────────────────────────────────────────────────────┐
│  Toolbar  │ Mode: [Edit][Compare] │                                          │
├───────────────────────────────────┬──────────────────────────────────────────┤
│                                   │                                          │
│   FEATURE BROWSER                 │            CANVAS                        │
│   (Scrollable grid)               │            (Large workspace)             │
│                                   │                                          │
│   [Layers list below]             ├──────────────────────────────────────────┤
│                                   │  Adjustment Bar (horizontal controls)    │
└───────────────────────────────────┴──────────────────────────────────────────┘
```

## Common File Structure (Per Layout)

Each layout folder contains:

### Pages (6 files)
| File | Description |
|------|-------------|
| `00-welcome.md` | Start screen, recent files |
| `01-editor.md` | Main composite editor |
| `02-compare.md` | Full compare mode (5 views) |
| `03-settings.md` | Application preferences |
| `04-help.md` | Help & documentation |
| `05-print-preview.md` | Print with reference comparison |

### Modals (7 files)
| File | Description |
|------|-------------|
| `new-composite.md` | Create new composite dialog |
| `export.md` | PNG/PDF export options |
| `case-info.md` | Case metadata editing |
| `shortcuts.md` | Keyboard shortcuts reference |
| `feature-detail.md` | Feature preview & customization |
| `confirm-dialogs.md` | Confirmation dialogs |
| `reference-upload.md` | Reference image upload/crop |

### Components (6 files)
| File | Description |
|------|-------------|
| `toolbar.md` | Main toolbar with menus |
| Layout-specific | Feature browser/sidebar/dock |
| `canvas.md` | Composite canvas with controls |
| Layout-specific | Layer panel/list |
| Layout-specific | Adjustment panel/bar |
| `compare-panel.md` | Compare mode controls |

### States (4 files)
| File | Description |
|------|-------------|
| `loading.md` | Loading spinners & progress |
| `empty.md` | Empty state illustrations |
| `error.md` | Error messages & recovery |
| `onboarding.md` | First-run tour & tooltips |

## Design Principles

### 1. Compare Mode Everywhere
Every editing view should support reference comparison:
- Toggle reference panel on/off
- Overlay reference on canvas
- Side-by-side split view
- Opacity control for overlay

### 2. Consistent Navigation
- Back arrow returns to previous screen
- Breadcrumbs for deep navigation
- Keyboard shortcuts for all actions

### 3. Progressive Disclosure
- Essential tools visible
- Advanced options in menus/panels
- Contextual help on hover

## Screen Flow

```
┌──────────────┐
│   Welcome    │
│   Screen     │
└──────┬───────┘
       │
       ▼
┌──────────────┐     ┌──────────────┐
│    Editor    │◄───►│   Compare    │
│    (Main)    │     │    Mode      │
└──────┬───────┘     └──────────────┘
       │
       ├──────► Export Modal
       ├──────► Settings Page
       ├──────► Case Info Modal
       ├──────► Print Preview
       └──────► Help Page
```

## Compare Mode Integration

All screens should have access to compare functionality:

| Screen | Compare Integration |
|--------|---------------------|
| Editor | Toggle compare panel, overlay mode |
| Compare | Full split-view with sync |
| Export | Side-by-side preview |
| Print | Include reference option |

---

**Last Updated**: 2025-01-01
