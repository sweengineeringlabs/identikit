# Component: Layer List (Layout D - Split View Focus)

## Layer List (Integrated in Left Panel)

```
──────────────────────────────────────
Layers
[👁][🔓] Hair
[👁][🔓] Eyes ●   ← Selected
[👁][🔓] Nose
[👁][🔓] Mouth
─────────────
[+] [📁] [🗑] 4 layers
```

## Layer Row Structure

```
[👁][🔓] Layer Name ●
 │   │       │      └─ Selection indicator
 │   │       └──────── Layer name
 │   └──────────────── Lock toggle
 └──────────────────── Visibility toggle
```

## Layer Row States

### Default
```
[👁][🔓] Hair
```

### Selected
```
[👁][🔓] Eyes ●   ← Highlighted row + dot
```

### Hidden
```
[👁̶][🔓] Nose     ← Visibility crossed out / dimmed
```

### Locked
```
[👁][🔒] Mouth    ← Lock icon filled
```

## Reordering (Drag)

```
Layers
[👁][🔓] Hair
         ╭────────────────╮
[👁][🔓] │ Eyes ●         │ ← Dragging layer
         ╰────────────────╯
         ↓ ← Drop indicator
[👁][🔓] Nose
[👁][🔓] Mouth
```

## Empty State

```
──────────────────────────────────────
Layers

        📑

   No layers yet

   Add features from
   categories above

─────────────
[+] [📁] [🗑] 0 layers
```

## Multi-Selection

```
Layers
[☑][👁][🔓] Hair      ← Checkbox visible
[☑][👁][🔓] Eyes ●
[☐][👁][🔓] Nose
[☐][👁][🔓] Mouth
─────────────
[Group] [🗑] 2 selected
```

## Grouped Layers

```
Layers
▾ 📁 Face Features
  [👁][🔓] Eyes ●
  [👁][🔓] Nose
  [👁][🔓] Mouth
[👁][🔓] Hair
─────────────
[+] [📁] [🗑] 4 layers
```

## Context Menu (Right-Click)

```
[👁][🔓] Eyes ●
              │
              ▼
      ┌─────────────────────┐
      │ Duplicate     Ctrl+D│
      │ Delete        Delete│
      │ ─────────────────── │
      │ Lock               │
      │ Hide               │
      │ ─────────────────── │
      │ Move to Top        │
      │ Move to Bottom     │
      │ ─────────────────── │
      │ Group with...    ► │
      └─────────────────────┘
```

## Bottom Controls

```
[+] [📁] [🗑] 4 layers
 │   │    │      │
 │   │    │      └─ Layer count
 │   │    └──────── Delete selected
 │   └───────────── Group selected
 └───────────────── Add new layer (opens feature browser)
```

## Actions

| Element | Action |
|---------|--------|
| [👁] | Toggle visibility |
| [🔓]/[🔒] | Toggle lock |
| Layer row click | Select layer |
| Layer row double-click | Rename layer |
| Layer row drag | Reorder |
| Layer row right-click | Context menu |
| [+] | Add new layer |
| [📁] | Group selected |
| [🗑] | Delete selected |
| ▾/▸ Group toggle | Expand/collapse group |

---

**Layout**: D (Split View Focus)
**Component**: `components/canvas/LayerList.tsx`
**Key Difference**: Compact list integrated below feature categories
