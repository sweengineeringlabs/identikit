# Component: Adjustment Controls (Layout C - Bottom Dock)

## Position Controls

```
Position
X: [____320____] px
Y: [____450____] px
[⌖ Center]

Input behavior:
- Direct number entry
- Arrow keys: ±1px
- Shift+Arrow: ±10px
- Tab to move between fields
```

## Scale Control

```
Scale
[━━━━━━━━━○━━━━━━━━] 100%
    25%          200%

Features:
- Drag handle to adjust
- Click track to jump
- Maintains aspect ratio
- Double-click to reset to 100%
```

## Rotation Control

```
Rotation
[____0____]° [↻ 90] [↺ 90]

Input behavior:
- Direct angle entry (-180 to 180)
- [↻ 90] rotates clockwise 90°
- [↺ 90] rotates counter-clockwise 90°
```

## Flip Controls

```
Flip
[↔ Horizontal] [↕ Vertical]

States:
[ ] Not flipped (default)
[●] Flipped active (highlighted)
```

## Opacity Control

```
Opacity
[━━━━━━━━━━━━━━━━━○] 100%
 0%                 100%

Features:
- Visual preview updates live
- Affects layer transparency
- Does not affect print output
```

## Compact Layout (Small Panel)

```
┌──────────────────┐
│ 👁️ Eyes          │
│ ────────────────  │
│ X:[320] Y:[450]  │
│ Scale: 100% [━○━]│
│ Rot: 0° [↻][↺]   │
│ Opa: 100% [━━○]  │
│ [↔][↕] [🔄Reset] │
└──────────────────┘
```

## Expanded Layout (Larger Panel)

```
┌───────────────────────────────────────────┐
│ 👁️ Eyes - Almond Style 1                  │
│ ─────────────────────────────────────────  │
│                                           │
│ Position                                  │
│ X: [_______320_______] px                 │
│ Y: [_______450_______] px                 │
│ [⌖ Center on Canvas]                      │
│                                           │
│ Size                                      │
│ W: [_______180_______] px  [🔗]           │
│ H: [________60_______] px                 │
│                                           │
│ Scale                                     │
│ [━━━━━━━━━━━━━━━━○━━━━━━━━━━━━━] 100%     │
│                                           │
│ Rotation                                  │
│ [_______0_______]°   [↻ +90°] [↺ -90°]    │
│                                           │
│ Flip                                      │
│ [↔ Horizontal]  [↕ Vertical]              │
│                                           │
│ Opacity                                   │
│ [━━━━━━━━━━━━━━━━━━━━━━━━━━━━━○] 100%     │
│                                           │
│ ─────────────────────────────────────────  │
│                          [🔄 Reset All]   │
└───────────────────────────────────────────┘
```

## Color Zone Controls

```
Color Zones
───────────────────

Iris
[■ #4A6B3D                           ▾]
Presets: [■][■][■][■][■][■][■][■][■]
         Brn Blu Grn Hzl Gry Blk Amb Vio

Sclera
[■ #FFFFFF                           ▾]

Outline
[■ #333333                           ▾]
Presets: [■][■][■]
         Dk  Med Lt

[🔄 Reset Colors]
```

## Color Picker Dropdown

```
[■ #4A6B3D ▾]
       │
       ▼
┌─────────────────────────┐
│ ┌─────────────────────┐ │
│ │                     │ │
│ │   [Color Gradient]  │ │
│ │                     │ │
│ └─────────────────────┘ │
│                         │
│ [━━━━━━○━━━━━━━━━━━━━]  │ <- Hue slider
│                         │
│ Hex: [#4A6B3D     ]     │
│ RGB: [74][107][61]      │
│                         │
│ Recent:                 │
│ [■][■][■][■][■][■]      │
│                         │
│ [Cancel] [Apply]        │
└─────────────────────────┘
```

## Slider Interaction States

```
Default:
[━━━━━━━━━○━━━━━━━━]

Hover:
[━━━━━━━━━◉━━━━━━━━]  <- Larger handle

Dragging:
[━━━━━━━━━●━━━━━━━━]  <- Filled handle
         ↑
        72%  <- Value tooltip
```

## Input Validation

```
Valid:
X: [____320____] px
    └─ Normal border

Invalid:
X: [____abc____] px
    └─ Red border
⚠️ Please enter a number

Out of Range:
Scale: [___5000___]%
⚠️ Maximum scale is 400%
```

## Multi-Selection Controls

```
┌──────────────────┐
│ 📑 3 layers      │
│ ────────────────  │
│                  │
│ Move Together    │
│ X: [+/- ____]    │
│ Y: [+/- ____]    │
│                  │
│ Scale All        │
│ [━━━━━━━○━] 100% │
│                  │
│ Align            │
│ [⫴] [⫿] [═⫻═]   │ <- Left, Center, Right
│ [⫴] [⫿] [║⫻║]   │ <- Top, Middle, Bottom
│                  │
│ Distribute       │
│ [⫴⫿⫴] [═⫻═]     │ <- Horizontal, Vertical
│                  │
│ [Group Selected] │
└──────────────────┘
```

## Actions

| Control | Action |
|---------|--------|
| Position inputs | Set absolute position |
| [⌖ Center] | Center on canvas |
| Scale slider | Proportional resize |
| Rotation input | Set rotation angle |
| [↻][↺] buttons | Quick 90° rotations |
| Flip buttons | Mirror horizontally/vertically |
| Opacity slider | Set layer transparency |
| Color pickers | Change zone colors |
| [🔄 Reset All] | Restore defaults |
| Align buttons | Align multiple selections |
| Distribute buttons | Even spacing |

---

**Layout**: C (Bottom Dock)
**Component**: `components/adjustment/AdjustmentControls.tsx`
**Note**: Controls adapt to panel width (compact vs expanded)
