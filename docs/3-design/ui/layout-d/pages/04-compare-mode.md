# Screen: Compare Mode (Layout D - Split View Focus)

## Compare Mode Active

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  File  Edit  View  Help                                              [_][□][✕]          │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│  [📄][💾][↩][↪] │ Mode: [Edit][Compare●] │ [Split●][Ovr][Sld][Onn][Diff] │ [Exit]      │
├────────────────────────────────────────┬────────────────────────────────────────────────┤
│                                        │                                                │
│  Reference Image                       │                                                │
│  ──────────────────────                │                                                │
│  ┌────────────────────────────────────┐│                                                │
│  │                                    ││                                                │
│  │                                    ││                                                │
│  │                                    ││                                                │
│  │          [REFERENCE]               ││          [COMPOSITE]                           │
│  │                                    ││                                                │
│  │                                    ││              👁️    👁️                          │
│  │                                    ││                                                │
│  │                                    ││                👃                              │
│  │                                    ││                                                │
│  │                                    ││                👄                              │
│  │                                    ││                                                │
│  └────────────────────────────────────┘│                                                │
│                                        │                                                │
│  witness_photo.jpg                     │                                                │
│  [🔄 Change] [🗑 Remove]               │                                                │
│                                        ├────────────────────────────────────────────────┤
│  ──────────────────────────────────────│ Split: [━━━━━○━━━━━] 50% │ [☑]Sync │ [⇄ Swap] │
│  Compare Settings                      ├────────────────────────────────────────────────┤
│  Orientation: [●Vert][○Horiz]          │                                                │
│  [☑] Sync zoom  [☑] Sync pan           │                                                │
│  [☑] Show labels                       │                                                │
│                                        │                                                │
└────────────────────────────────────────┴────────────────────────────────────────────────┘
```

## Overlay Mode

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  [📄][💾][↩][↪] │ Mode: [Edit][Compare●] │ [Split][Ovr●][Sld][Onn][Diff] │ [Exit]      │
├────────────────────────────────────────┬────────────────────────────────────────────────┤
│                                        │                                                │
│  Reference Image                       │                                                │
│  ┌────────────────────────────────────┐│                                                │
│  │       [REFERENCE THUMBNAIL]        ││                                                │
│  └────────────────────────────────────┘│                                                │
│  witness_photo.jpg                     │                                                │
│  [🔄 Change] [🗑 Remove]               │         [COMPOSITE + REFERENCE                 │
│                                        │              OVERLAID]                         │
│  ──────────────────────────────────────│                                                │
│                                        │                                                │
│  Overlay Settings                      │                                                │
│                                        │                                                │
│  Opacity                               │                                                │
│  [━━━━━━━━━━━○━━━━━] 50%              │                                                │
│                                        │                                                │
│  Blend Mode                            │                                                │
│  [Normal                          ▾]   │                                                │
│                                        │                                                │
│  Quick:                                ├────────────────────────────────────────────────┤
│  [0%][25%][50%][75%][100%]            │ Opacity: [━━━━○━━━] 50% │ [👁 Toggle] │ Key: R │
│                                        ├────────────────────────────────────────────────┤
│  [👁 Toggle Reference]     Key: R      │                                                │
│                                        │                                                │
└────────────────────────────────────────┴────────────────────────────────────────────────┘
```

## Slider Mode

```
┌────────────────────────────────────────┬────────────────────────────────────────────────┐
│                                        │                                                │
│  Reference Image                       │                              ┃                 │
│  [Thumbnail]                           │                              ┃                 │
│  witness_photo.jpg                     │      COMPOSITE               ┃    REFERENCE    │
│                                        │                              ┃                 │
│  ──────────────────────────────────────│          👁️    👁️            ┃                 │
│                                        │                              ┃                 │
│  Slider Settings                       │            👃                ┃◀─ Drag         │
│                                        │                              ┃                 │
│  Direction:                            │            👄                ┃                 │
│  [◀▶ Horizontal●]                      │                              ┃                 │
│  [▲▼ Vertical]                         │                              ┃                 │
│                                        │                              ┃                 │
│  Position                              │                              ┃                 │
│  [━━━━━━━○━━━━━━━━━] 50%              │                              ┃                 │
│                                        │                                                │
│  [☑] Animate                           ├────────────────────────────────────────────────┤
│                                        │ Slider: [━━━━━━○━━━━━━] 50% │ [☑]Animate      │
│                                        │                                                │
└────────────────────────────────────────┴────────────────────────────────────────────────┘
```

## Difference Mode

```
┌────────────────────────────────────────┬────────────────────────────────────────────────┐
│                                        │                                                │
│  Reference Image                       │                                                │
│  [Thumbnail]                           │                                                │
│  witness_photo.jpg                     │                                                │
│                                        │          [DIFFERENCE VIEW]                     │
│  ──────────────────────────────────────│                                                │
│                                        │      Areas of difference highlighted           │
│  Difference Settings                   │                                                │
│                                        │                                                │
│  Mode:                                 │                                                │
│  [●Absolute] [○Squared]                │                                                │
│                                        │                                                │
│  Amplify                               │                                                │
│  [━━━━━○━━━━━━━] 1.5×                 │                                                │
│                                        │                                                │
│  Threshold                             │                                                │
│  [━━━○━━━━━━━━━━] 5%                  ├────────────────────────────────────────────────┤
│                                        │ Legend: ░░▒▒▓▓██ │ Similarity: 72%            │
│  Color Map:                            ├────────────────────────────────────────────────┤
│  [●Grayscale][○Heat][○Highlight]       │                                                │
│                                        │                                                │
└────────────────────────────────────────┴────────────────────────────────────────────────┘
```

## No Reference Image

```
┌────────────────────────────────────────┬────────────────────────────────────────────────┐
│                                        │                                                │
│  Reference Image                       │                                                │
│  ──────────────────────                │                                                │
│                                        │                                                │
│            📷                          │                                                │
│                                        │                                                │
│    No reference image                  │              👁️    👁️                          │
│                                        │                                                │
│    Upload a witness photo or           │                👃                              │
│    sketch to compare with              │                                                │
│    your composite                      │                👄                              │
│                                        │                                                │
│    ┌────────────────────────────────┐  │                                                │
│    │     [📤 Upload Image]          │  │                                                │
│    └────────────────────────────────┘  │                                                │
│                                        │                                                │
│    Or drag an image here               │                                                │
│                                        │                                                │
│    Supported: JPG, PNG, BMP, TIFF      ├────────────────────────────────────────────────┤
│    Max size: 25MB                      │ Upload a reference image to enable comparison  │
│                                        │                                                │
└────────────────────────────────────────┴────────────────────────────────────────────────┘
```

## Actions

| Element | Action |
|---------|--------|
| Mode toggle | Switch Edit/Compare |
| Compare mode buttons | Select view mode |
| [Exit] | Return to edit mode |
| Opacity slider | Adjust reference transparency |
| Blend mode dropdown | Change overlay blend |
| Direction toggle | Slider/split orientation |
| Position slider | Move divider |
| [👁 Toggle] | Show/hide reference |
| R key | Quick toggle reference |
| [⇄ Swap] | Exchange positions |
| [Upload Image] | Add reference |
| [🔄 Change] | Replace reference |
| [🗑 Remove] | Delete reference |

---

**Layout**: D (Split View Focus)
**Component**: `CompareView.tsx`
**Key Difference**: Reference management in left panel, comparison on right
