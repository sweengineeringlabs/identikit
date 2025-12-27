# Component: Top Toolbar (Layout D - Split View Focus)

## Main Toolbar

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  [📄][💾][↩][↪] │ [✋][👆] │ Mode: [Edit●][Compare] │ Zoom:[100%▾] │ Grid[☑] Snap[☑]   │
└─────────────────────────────────────────────────────────────────────────────────────────┘
     File        Tools        Mode Toggle            Zoom            Options
```

## Mode Toggle (Key Feature)

```
Mode: [Edit●][Compare]
         │       │
         │       └─ Switch to compare mode
         └───────── Currently active (edit mode)

States:
[Edit●][Compare]  ← Edit mode active
[Edit][Compare●]  ← Compare mode active
```

## Edit Mode Toolbar

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  [📄][💾][↩][↪] │ [✋][👆●] │ Mode: [Edit●][Compare] │ Zoom:[100%▾] │ Grid[☑] Snap[☑]  │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Compare Mode Toolbar

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  [📄][💾][↩][↪] │ Mode: [Edit][Compare●] │ [Split●][Ovr][Sld][Onn][Diff] │ [Exit]      │
└─────────────────────────────────────────────────────────────────────────────────────────┘
                                            Compare mode buttons
```

## With Selection Active

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  [📄][💾][↩●][↪] │ [✋][👆●] │ Mode: [Edit●][Compare] │ Selected: Eyes │ [🗑][📋][⎘] │
└─────────────────────────────────────────────────────────────────────────────────────────┘
                   ↑                                                   ↑   ↑   ↑
              Undo available                                       Del Copy Dup
```

## File Menu

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

## Zoom Dropdown

```
Zoom:[100%▾]
         │
         ▼
  ┌─────────────┐
  │ 25%         │
  │ 50%         │
  │ 75%         │
  │ 100%    ✓   │
  │ 150%        │
  │ 200%        │
  │ ─────────── │
  │ Fit Screen  │
  │ Fill        │
  └─────────────┘
```

## Compare Mode Buttons

```
[Split●][Ovr][Sld][Onn][Diff]
   │      │    │    │    │
   │      │    │    │    └─ Difference view
   │      │    │    └────── Onion skin
   │      │    └─────────── Slider
   │      └──────────────── Overlay
   └─────────────────────── Split view (active)
```

## Actions

| Element | Action |
|---------|--------|
| 📄 | File menu |
| 💾 | Quick save |
| ↩/↪ | Undo/Redo |
| Tool buttons | Switch tool |
| Mode toggle | Switch Edit/Compare |
| Compare mode buttons | Select comparison type |
| Zoom dropdown | Change zoom level |
| Grid/Snap checkboxes | Toggle options |
| [Exit] | Exit compare mode |

---

**Layout**: D (Split View Focus)
**Component**: `components/toolbar/MainToolbar.tsx`
**Key Difference**: Prominent mode toggle for Edit/Compare switching
