# Component: Top Toolbar (Layout C - Bottom Dock)

## Main Toolbar

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  [📄][💾][↩][↪] │ [✋][👆][🔲] │ Zoom:[100%▾][⊞][1:1] │ Grid[☑] Snap[☑] │ 800×1000  │[👁 C]│
└─────────────────────────────────────────────────────────────────────────────────────────┘
     File          Tools           Zoom Controls           Options       Canvas    Compare
```

## Toolbar Sections Breakdown

### File Operations
```
┌─────────────┐
│ [📄][💾][↩][↪] │
└─────────────┘
  │   │   │  └─ Redo (Ctrl+Y)
  │   │   └──── Undo (Ctrl+Z)
  │   └──────── Save (Ctrl+S)
  └──────────── New/Open menu
```

### Tool Selection
```
┌─────────────┐
│ [✋][👆][🔲] │
└─────────────┘
  │   │   └─ Transform/Select tool (V)
  │   └───── Direct selection
  └──────── Pan tool (Space+drag)
```

### Zoom Controls
```
┌─────────────────────────┐
│ Zoom:[100%▾][⊞][1:1] │
└─────────────────────────┘
        │      │   └─ Actual size (Ctrl+1)
        │      └───── Fit to screen (Ctrl+0)
        └──────────── Zoom level dropdown
```

### Canvas Options
```
┌────────────────┐
│ Grid[☑] Snap[☑] │
└────────────────┘
       │      └─ Snap to grid (Shift+S)
       └──────── Toggle grid (G)
```

### Compare Toggle
```
┌───────┐
│ [👁 C] │
└───────┘
    └─ Toggle compare mode (C)
```

## With Selection Active

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  [📄][💾][↩][↪] │ [✋][👆●][🔲] │ Zoom:[100%▾] │ Selected: Eyes │ [🗑][📋][⎘][🔒] │ [👁 C] │
└─────────────────────────────────────────────────────────────────────────────────────────┘
                       ↑                              ↑        ↑   ↑   ↑   ↑
                   Active tool                   Selection   Del Cpy Dup Lock
```

## Zoom Dropdown

```
         [100%▾]
           │
           ▼
    ┌─────────────┐
    │ 25%         │
    │ 50%         │
    │ 75%         │
    │ 100%    ✓   │
    │ 150%        │
    │ 200%        │
    │ 400%        │
    │ ─────────── │
    │ Fit         │
    │ Fill        │
    └─────────────┘
```

## File Menu Expanded

```
[📄]
  │
  ▼
┌─────────────────────────────┐
│ 📄 New Composite    Ctrl+N  │
│ 📂 Open...          Ctrl+O  │
│ ─────────────────────────── │
│ Recent Files            ►   │
│ ─────────────────────────── │
│ 💾 Save             Ctrl+S  │
│ 💾 Save As...   Ctrl+Sh+S   │
│ ─────────────────────────── │
│ 📤 Export       Ctrl+Sh+E   │
│ 🖨️ Print            Ctrl+P  │
│ ─────────────────────────── │
│ ℹ️ Case Information  Ctrl+I │
└─────────────────────────────┘
```

## Undo/Redo States

### With History Available
```
[↩][↪]
 ↑   ↑
 │   └─ Redo enabled (has future states)
 └───── Undo enabled (has past states)
```

### No Undo Available
```
[↩][↪]
 ↑   ↑
 │   └─ Redo disabled (grayed out)
 └───── Undo disabled (grayed out, nothing to undo)
```

### Undo Tooltip
```
         [↩]
          │
          ▼
    ┌──────────────────────┐
    │ Undo: Move Eyes      │
    │ Ctrl+Z               │
    └──────────────────────┘
```

## Compare Mode Active

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  [📄][💾][↩][↪] │ Zoom:[100%▾] │ Compare Mode │ [Split●][Ovr][Sld][Onn][Diff] │ [Exit] │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Responsive Toolbar (Narrow Window)

```
┌───────────────────────────────────────────────────────────────────────┐
│ [📄][💾][↩↪] │ [Tools▾] │ [100%▾] │ [Options▾] │ [Compare] │
└───────────────────────────────────────────────────────────────────────┘
                    ↑            ↑
              Collapsed menus for narrow windows
```

## Tool States

| Tool | Icon | Active State | Hotkey |
|------|------|--------------|--------|
| Pan | ✋ | Highlighted background | Space (hold) |
| Select | 👆 | Highlighted background | V |
| Transform | 🔲 | Highlighted background | V (toggle) |

## Actions

| Element | Action |
|---------|--------|
| 📄 | File menu |
| 💾 | Quick save |
| ↩ | Undo last action |
| ↪ | Redo last undone |
| Tool buttons | Switch active tool |
| Zoom dropdown | Change zoom level |
| [⊞] | Fit canvas to view |
| [1:1] | Set 100% zoom |
| Grid checkbox | Toggle grid display |
| Snap checkbox | Toggle snap to grid |
| [👁 C] | Enter/exit compare mode |

---

**Layout**: C (Bottom Dock)
**Component**: `components/toolbar/MainToolbar.tsx`
**Note**: Toolbar is consistent across layouts; key difference is compare mode integration
