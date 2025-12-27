# Component: Compare Panel (Layout C - Bottom Dock)

## Compare Mode in Side Panel

```
┌──────────────────┐
│ [Comp●][Ref]     │  ← Compare tabs
├──────────────────┤
│                  │
│ Mode             │
│ [Split●]         │
│ [Overlay]        │
│ [Slider]         │
│ [Onion]          │
│ [Diff]           │
│                  │
│ ──────────────── │
│                  │
│ Split Settings   │
│ ●Vert ○Horiz     │
│                  │
│ Position         │
│ [━━━○━━━━] 50%   │
│                  │
│ [☑] Sync zoom    │
│ [☑] Sync pan     │
│ [☑] Labels       │
│                  │
│ [⇄ Swap Sides]   │
│                  │
└──────────────────┘
```

## Reference Tab

```
┌──────────────────┐
│ [Comp][Ref●]     │
├──────────────────┤
│                  │
│ Reference Image  │
│ ──────────────── │
│                  │
│ ┌──────────────┐ │
│ │              │ │
│ │  [Thumbnail] │ │
│ │              │ │
│ └──────────────┘ │
│ witness.jpg      │
│ 1920 × 1280      │
│                  │
│ [🔄 Change]      │
│ [🗑 Remove]      │
│                  │
│ ──────────────── │
│                  │
│ Alignment        │
│ [Auto ▾]         │
│                  │
│ Scale to fit     │
│ [━━━━━○━━] 100%  │
│                  │
└──────────────────┘
```

## Overlay Mode Settings

```
┌──────────────────┐
│ [Comp●][Ref]     │
├──────────────────┤
│                  │
│ Mode: [Overlay●] │
│                  │
│ ──────────────── │
│                  │
│ Reference Opacity│
│ [━━━━━━○━━] 50%  │
│                  │
│ Blend Mode       │
│ [Normal      ▾]  │
│                  │
│ Quick Opacity    │
│ [0%][25%][50%]   │
│ [75%][100%]      │
│                  │
│ ──────────────── │
│                  │
│ [👁 Toggle Ref]  │
│ Hotkey: R        │
│                  │
└──────────────────┘
```

## Difference Mode Settings

```
┌──────────────────┐
│ [Comp●][Ref]     │
├──────────────────┤
│                  │
│ Mode: [Diff●]    │
│                  │
│ ──────────────── │
│                  │
│ Diff Type        │
│ ●Absolute        │
│ ○Squared         │
│                  │
│ Amplify          │
│ [━━━○━━━━] 1.5×  │
│                  │
│ Threshold        │
│ [━○━━━━━━] 5%    │
│                  │
│ Color Map        │
│ ●Grayscale       │
│ ○Heat            │
│ ○Highlight       │
│                  │
│ ──────────────── │
│                  │
│ Similarity: 72%  │
│                  │
│ Legend:          │
│ ░░▒▒▓▓██ 0→100% │
│                  │
└──────────────────┘
```

## Slider Mode Settings

```
┌──────────────────┐
│ [Comp●][Ref]     │
├──────────────────┤
│                  │
│ Mode: [Slider●]  │
│                  │
│ ──────────────── │
│                  │
│ Direction        │
│ [◀▶ Horiz●]      │
│ [▲▼ Vertical]    │
│                  │
│ Position         │
│ [━━━━━○━━━] 50%  │
│                  │
│ [☑] Animate      │
│                  │
│ ──────────────── │
│                  │
│ Tip: Drag the    │
│ divider on the   │
│ canvas to reveal │
│                  │
└──────────────────┘
```

## No Reference Uploaded

```
┌──────────────────┐
│ [Comp][Ref●]     │
├──────────────────┤
│                  │
│       📷         │
│                  │
│ No reference     │
│ image            │
│                  │
│ Upload an image  │
│ to compare with  │
│ your composite   │
│                  │
│ [📤 Upload]      │
│                  │
│ Supported:       │
│ JPG, PNG, BMP    │
│ Max: 25MB        │
│                  │
└──────────────────┘
```

## Toolbar Integration

When compare mode is active, the toolbar shows mode buttons:

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  [📄][💾][↩][↪] │ Compare: │ [Split●][Ovr][Sld][Onn][Dif] │ [Sync☑] │ [Exit Compare] │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Compare Canvas Views

### Split View
```
┌───────────────────────────────┬───────────────────────────────┐
│                               │                               │
│        COMPOSITE              │         REFERENCE             │
│                               │                               │
│          👁️    👁️             │          [Photo]              │
│                               │                               │
│            👃                 │                               │
│                               │                               │
│            👄                 │                               │
│                               │                               │
└───────────────────────────────┴───────────────────────────────┘
```

### Slider View
```
┌─────────────────────────────────────────────────────────────────┐
│                              ┃                                  │
│        COMPOSITE             ┃           REFERENCE              │
│                              ┃                                  │
│          👁️    👁️             ┃                                  │
│                              ┃◀────── Drag to reveal            │
│            👃                 ┃                                  │
│                              ┃                                  │
│            👄                 ┃                                  │
│                              ┃                                  │
└─────────────────────────────────────────────────────────────────┘
```

## Actions

| Element | Action |
|---------|--------|
| Mode buttons | Switch compare mode |
| Opacity slider | Adjust reference visibility |
| Blend mode dropdown | Change overlay blend |
| Quick opacity buttons | Jump to preset values |
| [👁 Toggle Ref] | Show/hide reference |
| R key | Quick toggle reference |
| Direction toggle | Slider orientation |
| Position slider | Move split/slider |
| [☑] Sync zoom/pan | Link canvas navigation |
| [⇄ Swap Sides] | Exchange positions |
| [📤 Upload] | Add reference image |
| [🔄 Change] | Replace reference |
| [🗑 Remove] | Delete reference |
| [Exit Compare] | Leave compare mode |

---

**Layout**: C (Bottom Dock)
**Component**: `components/compare/ComparePanel.tsx`
**Key Difference**: Compare controls integrated into compact side panel tabs
