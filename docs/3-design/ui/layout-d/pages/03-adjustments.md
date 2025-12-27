# Screen: Adjustments (Layout D - Split View Focus)

## Adjustment Bar (Below Canvas)

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ Selected: Eyes │ X:[320] Y:[450] │ Scale:[━━○━] 100% │ Rot:[0°][↻][↺] │ Opa:[━━━○] │[🔄]│
└────────────────────────────────────────────────────────────────────────────────────────┘
     ↑                ↑                    ↑                  ↑              ↑        ↑
  Layer name      Position             Scale slider      Rotation       Opacity   Reset
```

## Expanded Adjustment Bar

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ Eyes - Almond Style 1                                                                  │
├────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                        │
│ Position              Scale                 Rotation              Transform            │
│ X: [____320____]     [━━━━━━━━○━━━] 100%   [____0____]°         [↔ Flip H]           │
│ Y: [____450____]                           [↻ +90] [↺ -90]      [↕ Flip V]           │
│ [⌖ Center]                                                                            │
│                                                                                        │
│ Opacity               Color Zones                                                      │
│ [━━━━━━━━━━━━━○] 100% Iris: [■ #4A6B3D ▾]  Sclera: [■ #FFF ▾]  Outline: [■ #333 ▾]  │
│                                                                                        │
│                                                              [🔄 Reset All]           │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

## Full Layout with Adjustments

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  Toolbar: Mode: [Edit●][Compare]                                                        │
├────────────────────────────────────────┬────────────────────────────────────────────────┤
│                                        │                                                │
│  [🔍 Search...]                        │                                                │
│                                        │                                                │
│  ▾ Hair (24)                           │                                                │
│  [Feature grid...]                     │              ○───────────────○                 │
│                                        │              │               │                 │
│  ▾ Eyes (32)                           │              │  👁️       👁️   │ ← Selected     │
│  [Feature grid...]                     │              │               │                 │
│                                        │              ○───────────────○                 │
│  ▸ Nose (20)                           │                     ○                          │
│  ▸ Mouth (16)                          │                     ↺                          │
│  ▸ More...                             │                                                │
│                                        │                    👃                          │
│  ──────────────────────────────────────│                                                │
│  Layers                                │                    👄                          │
│  [👁] Hair                             │                                                │
│  [👁] Eyes ● ←Selected                 ├────────────────────────────────────────────────┤
│  [👁] Nose                             │ Eyes │ X:[320] Y:[450] │ [━○━]100% │ [🔄 Reset]│
│  [👁] Mouth                            ├────────────────────────────────────────────────┤
│  [+][📁][🗑]                           │ [↔ H][↕ V] │ Iris:[■▾] Sclera:[■▾] Out:[■▾]   │
│                                        │                                                │
└────────────────────────────────────────┴────────────────────────────────────────────────┘
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
│ 3 layers selected │ Move: X:[±__] Y:[±__] │ Scale:[━━○━━] │ Align:[⫴][⫿][═] │ [Group] │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

## Alignment Tools (Multi-Selection)

```
Align:     [⫴] [⫿] [═⫻═]     Distribute:  [⫴⫿⫴] [═⫻═]
            │   │    │                      │      │
           Left Center Right          Horizontal Vertical

           [⫴] [⫿] [║⫻║]
            │   │    │
           Top Middle Bottom
```

## Color Zone Picker

```
Iris: [■ #4A6B3D ▾]
              │
              ▼
┌───────────────────────────────────┐
│  Preset Colors                    │
│  [■][■][■][■][■][■][■][■][■]     │
│  Brn Blu Grn Hzl Gry Blk Amb Vio │
│                                   │
│  Custom: [#4A6B3D    ] [Apply]    │
│                                   │
│  Recent: [■][■][■]                │
└───────────────────────────────────┘
```

## Compact vs Expanded Toggle

```
Compact (default):
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ Eyes │ X:[320] Y:[450] │ [━○━]100% │ Rot:[0°] │ [━━━○]100% │ [↔][↕] │ [🔄] │ [▼ More] │
└────────────────────────────────────────────────────────────────────────────────────────┘
                                                                              ↑
                                                                        Expand button

Expanded:
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ Eyes - Almond Style 1                                                      [▲ Less]   │
├────────────────────────────────────────────────────────────────────────────────────────┤
│ [Full adjustment controls...]                                                          │
│                                                                                        │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

## Actions

| Element | Action |
|---------|--------|
| Position inputs | Set X/Y coordinates |
| [⌖ Center] | Center on canvas |
| Scale slider | Resize feature |
| Rotation input | Set angle |
| [↻][↺] | Rotate 90° |
| [↔ H][↕ V] | Flip |
| Opacity slider | Set transparency |
| Color pickers | Change zone colors |
| [🔄 Reset All] | Reset to defaults |
| [▼ More]/[▲ Less] | Toggle expanded view |
| Align buttons | Align selection |
| [Group] | Group selected layers |

---

**Layout**: D (Split View Focus)
**Component**: `AdjustmentBar.tsx`
**Key Difference**: Horizontal bar below canvas, compact by default
