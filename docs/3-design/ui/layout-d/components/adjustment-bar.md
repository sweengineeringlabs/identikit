# Component: Adjustment Bar (Layout D - Split View Focus)

## Compact Adjustment Bar (Below Canvas)

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ Eyes │ X:[320] Y:[450] │ Scale:[━━○━] 100% │ Rot:[0°][↻][↺] │ Opa:[━━━○] 100% │ [🔄]  │
└────────────────────────────────────────────────────────────────────────────────────────┘
   ↑         ↑                  ↑                  ↑               ↑              ↑
  Name   Position            Scale              Rotation        Opacity         Reset
```

## Bar Sections

### Layer Name
```
Eyes │
  ↑
Selected layer name
(click to see full: "Eyes - Almond Style 1")
```

### Position
```
X:[320] Y:[450]
   ↑       ↑
   └───────┴── Editable inputs
```

### Scale Slider
```
Scale:[━━━━━━━○━━━━] 100%
             ↑
      Drag to resize
```

### Rotation
```
Rot:[0°][↻][↺]
      ↑   ↑  ↑
      │   │  └─ Rotate CCW 90°
      │   └──── Rotate CW 90°
      └──────── Direct input
```

### Opacity
```
Opa:[━━━━━━━━━○] 100%
              ↑
      Drag to adjust
```

### Reset Button
```
[🔄]
  ↑
Reset all properties to defaults
```

## Expanded Bar (Click "More" or resize)

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ Eyes - Almond Style 1                                                      [▲ Less]   │
├────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                        │
│ Position              Size                  Transform                                  │
│ X: [____320____]     W: [____180____]       Scale: [━━━━━━━━○━━━] 100%               │
│ Y: [____450____]     H: [_____60____] [🔗]  Rotation: [____0____]° [↻][↺]            │
│ [⌖ Center]                                  [↔ Flip H] [↕ Flip V]                     │
│                                                                                        │
│ Opacity                    Color Zones                                                 │
│ [━━━━━━━━━━━━━━━○] 100%   Iris: [■ #4A6B3D ▾]  Sclera: [■ #FFFFFF ▾]                 │
│                            Outline: [■ #333333 ▾]                                      │
│                                                                                        │
│                                                                      [🔄 Reset All]   │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

## No Selection State

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ No layer selected │ Click a layer or canvas object to adjust                          │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

## Multi-Selection State

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ 3 layers │ Move: X:[±___] Y:[±___] │ Scale:[━━━○━━] │ Align:[⫴][⫿][═] │ [Group]       │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

## Compare Mode Adjustments

When in compare mode:

### Split Mode
```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ Split: [━━━━━○━━━━━] 50% │ [☑]Sync Zoom │ [☑]Sync Pan │ [⇄ Swap] │ [●Vert][○Horiz]   │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

### Overlay Mode
```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ Opacity: [━━━━━━━○━━━━━] 50% │ Blend: [Normal ▾] │ [👁 Toggle Reference] Key: R        │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

### Slider Mode
```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ Slider: [━━━━━○━━━━━━━━] 50% │ Direction: [◀▶ Horizontal●] [▲▼ Vertical] │ [☑]Animate │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

### Difference Mode
```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ Amplify:[━━○━━]1.5× │ Threshold:[━○━━]5% │ [●Gray][○Heat] │ Legend:░▒▓█ │ Match: 72% │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

## Actions

| Element | Action |
|---------|--------|
| Position inputs | Set X/Y coordinates |
| Scale slider | Resize feature |
| Rotation input | Set angle |
| [↻][↺] buttons | Rotate 90° |
| Opacity slider | Set transparency |
| [↔ H][↕ V] | Flip |
| [🔄] / [🔄 Reset All] | Reset to defaults |
| [▲ Less] / [▼ More] | Toggle expanded view |
| Color pickers | Change zone colors |
| [⌖ Center] | Center on canvas |
| Align buttons | Align multi-selection |
| [Group] | Group selected |
| Compare controls | Adjust comparison settings |

---

**Layout**: D (Split View Focus)
**Component**: `components/adjustment/AdjustmentBar.tsx`
**Key Difference**: Horizontal bar below canvas, expands on demand
