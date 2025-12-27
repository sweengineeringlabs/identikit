# Component: Floating Panel (Layout B - Floating Panels)

## Panel Structure

```
┌─Panel Title────────────────[_][□][✕]┐
│                                     │
│  [Panel Content]                    │
│                                     │
│                                     │
└─────────────────────────────────────┘
  ↑                            ↑ ↑ ↑
  Drag to move          Min Max Close
```

## Window Controls

```
[_] Minimize - Collapse to title bar only
[□] Maximize - Expand to full screen
[✕] Close - Hide panel (reopen from toolbar)
```

## Panel States

### Normal
```
┌─Features─────────────[_][□][✕]┐
│ [🔍 Search...]                │
│ ▸ Hair    (24)                │
│ ▸ Eyes    (32)                │
│ ▸ Nose    (20)                │
│ ...                           │
└───────────────────────────────┘
```

### Focused (Active)
```
╔═Features═════════════[_][□][✕]╗
║ [🔍 Search...]                ║  ← Blue border
║ ▸ Hair    (24)                ║
║ ▸ Eyes    (32)                ║
║ ...                           ║
╚═══════════════════════════════╝
```

### Collapsed
```
┌─Features─────────────[▲][□][✕]┐
└───────────────────────────────┘
```

### Maximized
```
┌─Features────────────────────────────────────────────────[_][❐][✕]┐
│                                                                  │
│  [Full screen panel content]                                     │
│                                                                  │
│                                                                  │
│                                                                  │
│                                                                  │
│                                                                  │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

## Resizing

```
┌─Panel────────────┐
│                  │←→ Drag right edge
│                  │
│                  │
└────────────↘─────┘
           ↑
    Drag corner to resize
    both dimensions
```

## Docking Behavior

```
Panel dragged to edge:

  ┌────────────────────────────────────────────────────┐
  │                                                    │
╔═╪═══════════════╗                                    │
║ │  Features     ║  ← Snap indicator appears          │
║ │  [Will dock]  ║                                    │
╚═╪═══════════════╝                                    │
  │                                                    │
  │                    [Canvas]                        │
  │                                                    │
  └────────────────────────────────────────────────────┘
```

## Stacked Panels

```
┌─Features─────────[✕]┐
│ ▸ Hair    (24)      │
│ ▸ Eyes    (32)      │
└──────┬──────────────┘
       │
   ┌─Layers────────[✕]┐
   │ [👁] Hair         │  ← Panels can overlap
   │ [👁] Eyes ●       │
   └───────────────────┘
```

## Panel Menu (Right-click title)

```
┌─Features─────────[✕]┐
│ ┌────────────────────────┐
│ │ Minimize               │
│ │ Maximize               │
│ │ Close                  │
│ │ ───────────────────── │
│ │ Reset Position         │
│ │ Reset Size             │
│ │ ───────────────────── │
│ │ Always on Top      [☐] │
│ │ Transparent        [☐] │
│ └────────────────────────┘
```

## Feature Library Panel

```
┌─Features─────────────[_][□][✕]┐
│ [🔍 Search features...]       │
│                               │
│ ▸ Hair         (24)           │
│ ▾ Eyes         (32)           │
│   ┌───┐┌───┐┌───┐            │
│   │👁️👁️││👁️👁️││👁️👁️│            │
│   └───┘└───┘└───┘            │
│   ┌───┐┌───┐┌───┐            │
│   │👁️👁️││👁️👁️││👁️👁️│            │
│   └───┘└───┘└───┘            │
│ ▸ Eyebrows     (18)           │
│ ▸ Nose         (20)           │
│ ▸ Mouth        (16)           │
│ ▸ Accessories  (28)           │
│                               │
└───────────────────────────────┘
```

## Adjustments Panel

```
┌─Adjustments──────────[_][□][✕]┐
│ 👁️ Eyes - Almond Style 1       │
│ ─────────────────────────     │
│                               │
│ Position                      │
│ X: [___320___] Y: [___450___] │
│                               │
│ Scale: [━━━━━━━○━━] 100%      │
│ Rotation: [___0___]°          │
│ Opacity: [━━━━━━━━━○] 100%    │
│                               │
│ [🔄 Reset All]                │
│                               │
└───────────────────────────────┘
```

## Layers Panel

```
┌─Layers───────────────[_][□][✕]┐
│ [+] [📁 Group] [🗑]           │
│                               │
│ ┌───────────────────────────┐ │
│ │ [👁][🔓] ┌───┐ Hair       │ │
│ │         │   │             │ │
│ │         └───┘             │ │
│ ├───────────────────────────┤ │
│ │ [👁][🔓] ┌───┐ Eyes ●     │ │
│ │         │👁️👁️│             │ │
│ │         └───┘             │ │
│ ├───────────────────────────┤ │
│ │ [👁][🔓] ┌───┐ Nose       │ │
│ │         │ 👃 │             │ │
│ │         └───┘             │ │
│ └───────────────────────────┘ │
│                               │
│ 5 layers                      │
└───────────────────────────────┘
```

---

**Layout**: B (Floating Panels)
**Component**: `components/ui/FloatingPanel.tsx`
**Props**: `title`, `visible`, `position`, `size`, `onMove`, `onResize`, `onClose`, `onMinimize`, `onMaximize`
