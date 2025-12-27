# Component: Side Panel (Layout C - Bottom Dock)

## Tabbed Side Panel

```
┌──────────────────┐
│ [Layers][Adjust] │  ← Tab row
├──────────────────┤
│                  │
│  [Tab content]   │
│                  │
│                  │
│                  │
│                  │
│                  │
│                  │
│                  │
│                  │
│                  │
│                  │
└──────────────────┘
```

## Layers Tab Active

```
┌──────────────────┐
│ [Layers●][Adjust]│
├──────────────────┤
│                  │
│ [👁][🔓] Hair    │
│ [👁][🔓] Eyes ●  │ ← Selected layer
│ [👁][🔓] Nose    │
│ [👁][🔓] Mouth   │
│                  │
│ ──────────────── │
│ 4 layers         │
│                  │
│ [+] [📁] [🗑]    │
│                  │
└──────────────────┘
```

## Layers Tab - Detailed

```
┌──────────────────┐
│ [Layers●][Adjust]│
├──────────────────┤
│                  │
│ ┌──────────────┐ │
│ │[👁][🔓] Hair │ │  <- Layer row
│ │  └─ Opacity: │ │     Expanded info
│ │     100%     │ │
│ └──────────────┘ │
│                  │
│ ┌──────────────┐ │
│ │[👁][🔓]Eyes●│ │  <- Selected (highlighted)
│ │  Almond-001  │ │
│ │  100%        │ │
│ └──────────────┘ │
│                  │
│ ┌──────────────┐ │
│ │[👁][🔓] Nose │ │
│ └──────────────┘ │
│                  │
│ ┌──────────────┐ │
│ │[👁][🔓]Mouth │ │
│ └──────────────┘ │
│                  │
│ ──────────────── │
│ 4 layers         │
│ [+] [📁] [🗑]    │
└──────────────────┘
```

## Adjust Tab Active

```
┌──────────────────┐
│ [Layers][Adjust●]│
├──────────────────┤
│ 👁️ Eyes-Almond   │
│ ────────────────  │
│                  │
│ Position         │
│ X: [____320____] │
│ Y: [____450____] │
│ [⌖ Center]       │
│                  │
│ Scale            │
│ [━━━━━━━○━] 100% │
│                  │
│ Rotation         │
│ [__0__]° [↻][↺]  │
│                  │
│ Flip             │
│ [↔ H]  [↕ V]     │
│                  │
│ Opacity          │
│ [━━━━━━━━━○] 100%│
│                  │
│ [🔄 Reset All]   │
└──────────────────┘
```

## Adjust Tab - With Color Zones

```
┌──────────────────┐
│ [Layers][Adjust●]│
├──────────────────┤
│ 👁️ Eyes-Almond   │
│ ────────────────  │
│                  │
│ ▸ Position       │
│ ▸ Transform      │
│ ▾ Color Zones    │
│   ──────────────  │
│                  │
│   Iris           │
│   [■ #4A6B3D ▾]  │
│   [■][■][■][■]   │
│                  │
│   Sclera         │
│   [■ #FFFFFF ▾]  │
│                  │
│   Outline        │
│   [■ #333333 ▾]  │
│                  │
│ [🔄 Reset Colors]│
└──────────────────┘
```

## Compare Tab (When Compare Mode Active)

```
┌──────────────────┐
│ [Comp][Ref●]     │
├──────────────────┤
│                  │
│ Reference Image  │
│ ──────────────── │
│ witness.jpg      │
│                  │
│ [🔄 Change]      │
│ [🗑 Remove]      │
│                  │
│ ──────────────── │
│                  │
│ Compare Settings │
│                  │
│ Opacity          │
│ [━━━━━○━━━] 50%  │
│                  │
│ Blend Mode       │
│ [Normal      ▾]  │
│                  │
│ Quick:           │
│ [0][25][50]      │
│ [75][100]        │
│                  │
└──────────────────┘
```

## No Selection State

```
┌──────────────────┐
│ [Layers][Adjust●]│
├──────────────────┤
│                  │
│       ◇          │
│                  │
│  No selection    │
│                  │
│  Select a layer  │
│  to adjust its   │
│  properties      │
│                  │
│  • Click canvas  │
│  • Click layer   │
│                  │
│                  │
│                  │
│                  │
│                  │
│                  │
│                  │
│                  │
└──────────────────┘
```

## Empty Layers

```
┌──────────────────┐
│ [Layers●][Adjust]│
├──────────────────┤
│                  │
│       📑         │
│                  │
│  No layers yet   │
│                  │
│  Drag features   │
│  from the dock   │
│  below to add    │
│                  │
│  Or use [+] to   │
│  browse          │
│                  │
│ ──────────────── │
│ 0 layers         │
│                  │
│ [+] [📁] [🗑]    │
│                  │
└──────────────────┘
```

## Layer Controls

```
Layer Row:
┌────────────────────────────┐
│ [👁] [🔓] ┌──┐ Layer Name  │
│           │  │             │
│           └──┘             │
└────────────────────────────┘
  │    │     ↑
  │    │     Thumbnail
  │    └─ Lock toggle
  └────── Visibility toggle

Bottom Controls:
[+] [📁] [🗑]
 │   │    └─ Delete selected layer
 │   └────── Group selected layers
 └────────── Add new layer (opens feature browser)
```

## Actions

| Element | Action |
|---------|--------|
| Tab buttons | Switch panel content |
| [👁] | Toggle layer visibility |
| [🔓] | Toggle layer lock |
| Layer row click | Select layer |
| Layer row drag | Reorder layers |
| Position inputs | Set X/Y coordinates |
| [⌖ Center] | Center on canvas |
| Scale slider | Resize feature |
| Rotation input | Set rotation angle |
| [↻][↺] | Rotate 90° |
| [↔ H][↕ V] | Flip horizontally/vertically |
| Opacity slider | Adjust transparency |
| Color swatches | Change zone colors |
| [🔄 Reset All] | Reset to defaults |
| [+] | Add layer |
| [📁] | Group layers |
| [🗑] | Delete layer |

---

**Layout**: C (Bottom Dock)
**Component**: `components/layout/SidePanel.tsx`
**Key Difference**: Combined Layers/Adjustments in single compact panel
