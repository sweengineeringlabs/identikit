# Component: Compare View (Layout D - Split View Focus)

## Compare Mode Active

When mode is switched to Compare, the layout reconfigures:

```
┌────────────────────────────────────────┬────────────────────────────────────────────────┐
│                                        │                                                │
│  Reference Management                  │           Comparison Canvas                    │
│  (Left panel switches content)         │           (Shows comparison view)              │
│                                        │                                                │
└────────────────────────────────────────┴────────────────────────────────────────────────┘
```

## Left Panel in Compare Mode

```
┌────────────────────────────────────────┐
│                                        │
│  Reference Image                       │
│  ──────────────────────                │
│  ┌────────────────────────────────────┐│
│  │                                    ││
│  │       [REFERENCE THUMBNAIL]        ││
│  │                                    ││
│  └────────────────────────────────────┘│
│  witness_photo.jpg                     │
│  1920 × 1280 px                        │
│                                        │
│  [🔄 Change] [🗑 Remove]               │
│                                        │
│  ──────────────────────────────────────│
│                                        │
│  Compare Settings                      │
│                                        │
│  [Mode-specific settings...]           │
│                                        │
│  ──────────────────────────────────────│
│                                        │
│  Layers (still accessible)             │
│  [👁] Hair                             │
│  [👁] Eyes ●                           │
│  [👁] Nose                             │
│  [👁] Mouth                            │
│                                        │
└────────────────────────────────────────┘
```

## Split View Settings (Left Panel)

```
Compare Settings
───────────────────

Mode: Split●

Orientation
[● Vertical]  [○ Horizontal]

Split Position
[━━━━━━━○━━━━━━━] 50%

Options
[☑] Sync zoom between views
[☑] Sync pan between views
[☑] Show labels

[⇄ Swap Composite/Reference]
```

## Overlay Settings (Left Panel)

```
Compare Settings
───────────────────

Mode: Overlay●

Reference Opacity
[━━━━━━━━━━○━━━━━━━━] 50%

Blend Mode
[Normal                     ▾]

Quick Opacity
[0%] [25%] [50%] [75%] [100%]

──────────────────

[👁 Toggle Reference]
Keyboard: R
```

## Slider Settings (Left Panel)

```
Compare Settings
───────────────────

Mode: Slider●

Direction
[◀▶ Horizontal ●]
[▲▼ Vertical]

Position
[━━━━━━━━━○━━━━━━━] 50%

Options
[☑] Auto-animate on hover

Tip: Drag the divider on
the canvas to reveal
```

## Difference Settings (Left Panel)

```
Compare Settings
───────────────────

Mode: Difference●

Diff Algorithm
[● Absolute]
[○ Squared]

Amplify
[━━━━━━○━━━━━━━━] 1.5×

Threshold
[━━━○━━━━━━━━━━━] 5%

Color Map
[● Grayscale]
[○ Heat Map]
[○ Highlight]

──────────────────

Legend
░░░░▒▒▒▒▓▓▓▓████
0%            100%

Similarity Score
█████████████░░░ 72%
```

## Canvas Area in Compare Mode

### Split View
```
┌──────────────────────────────────────┬─────────────────────────────────────────────────┐
│                                      │                                                 │
│         Composite                    │            Reference                            │
│                                      │                                                 │
│           👁️    👁️                    │                                                 │
│                                      │              [Photo]                            │
│             👃                        │                                                 │
│                                      │                                                 │
│             👄                        │                                                 │
│                                      │                                                 │
└──────────────────────────────────────┴─────────────────────────────────────────────────┘
```

### Overlay View
```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                                                                                        │
│                                                                                        │
│                           Composite + Reference (overlaid)                             │
│                                                                                        │
│                              [Blended image view]                                      │
│                                                                                        │
│                                                                                        │
│                                                                                        │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

### Slider View
```
┌────────────────────────────────────────┃────────────────────────────────────────────────┐
│                                        ┃                                                │
│         Composite                      ┃            Reference                           │
│                                        ┃                                                │
│           👁️    👁️                      ┃                                                │
│                                        ┃◀── Draggable divider                           │
│             👃                          ┃                                                │
│                                        ┃                                                │
│             👄                          ┃                                                │
│                                        ┃                                                │
└────────────────────────────────────────┃────────────────────────────────────────────────┘
```

## No Reference State

```
┌────────────────────────────────────────┐
│                                        │
│  Reference Image                       │
│  ──────────────────────                │
│                                        │
│            📷                          │
│                                        │
│    No reference image                  │
│                                        │
│    Upload a witness photo              │
│    to compare with your                │
│    composite                           │
│                                        │
│    ┌────────────────────────────────┐  │
│    │     [📤 Upload Image]          │  │
│    └────────────────────────────────┘  │
│                                        │
│    Or drag an image here               │
│                                        │
└────────────────────────────────────────┘
```

## Actions

| Element | Action |
|---------|--------|
| Mode buttons (toolbar) | Switch comparison mode |
| [📤 Upload Image] | Add reference |
| [🔄 Change] | Replace reference |
| [🗑 Remove] | Delete reference |
| Opacity slider | Adjust transparency |
| Blend mode dropdown | Change blend mode |
| Direction toggle | Switch orientation |
| Position slider | Move divider |
| [☑] Options | Toggle sync/animate |
| [⇄ Swap] | Exchange positions |
| R key | Quick toggle reference |
| Canvas divider | Drag to reveal |

---

**Layout**: D (Split View Focus)
**Component**: `components/compare/CompareView.tsx`
**Key Difference**: Left panel switches to reference management in compare mode
