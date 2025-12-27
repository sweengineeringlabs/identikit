# Component: Compare Panel (Layout B - Floating Panels)

## Floating Compare Controls

```
┌─Compare────────────────────────────────────────────[_][□][✕]┐
│                                                             │
│  Mode: [Split ●] [Overlay] [Slider] [Onion] [Diff]         │
│                                                             │
│  ─────────────────────────────────────────────────────────  │
│                                                             │
│  Reference: witness_photo.jpg                               │
│  [🔄 Change] [🗑 Remove]                                    │
│                                                             │
│  ─────────────────────────────────────────────────────────  │
│                                                             │
│  Split Settings                                             │
│  Orientation: [● Vertical] [○ Horizontal]                   │
│  Split Position: [━━━━━━━━━○━━━━━━━━] 50%                  │
│                                                             │
│  [☑] Sync zoom      [⇄ Swap Sides]                         │
│  [☑] Sync pan                                               │
│  [☑] Show labels                                            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Overlay Mode Settings

```
┌─Compare────────────────────────────────────────────[_][□][✕]┐
│                                                             │
│  Mode: [Split] [Overlay ●] [Slider] [Onion] [Diff]         │
│                                                             │
│  ─────────────────────────────────────────────────────────  │
│                                                             │
│  Reference Opacity                                          │
│  [━━━━━━━━━━━━━━━○━━━━━━━━━━━━━━] 50%                       │
│                                                             │
│  Blend Mode: [Normal                               ▾]       │
│                                                             │
│  Quick Opacity: [0%] [25%] [50%] [75%] [100%]              │
│                                                             │
│  [👁 Toggle Reference]                          Hotkey: R   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Difference Mode Settings

```
┌─Compare────────────────────────────────────────────[_][□][✕]┐
│                                                             │
│  Mode: [Split] [Overlay] [Slider] [Onion] [Diff ●]         │
│                                                             │
│  ─────────────────────────────────────────────────────────  │
│                                                             │
│  Mode: [● Absolute] [○ Squared]                             │
│                                                             │
│  Amplify: [━━━━━━━━━○━━━━━━━━] 1.5x                        │
│  Threshold: [━━━○━━━━━━━━━━━━━] 5%                         │
│                                                             │
│  Color Map: [● Grayscale] [○ Heat] [○ Highlight]           │
│                                                             │
│  Legend: ░░░░░▒▒▒▒▓▓▓▓████  (0% → 100%)                    │
│                                                             │
│  Similarity Score: 72%                                      │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## No Reference Image

```
┌─Compare────────────────────────────────────────────[_][□][✕]┐
│                                                             │
│                         📷                                  │
│                                                             │
│              No reference image                             │
│                                                             │
│   Add a reference image to compare with your composite      │
│                                                             │
│                  [📤 Upload Image]                          │
│                                                             │
│              Or drag an image here                          │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Compact Mode

```
┌─Compare──[✕]┐
│[Split●][Ovr]│
│[Sldr][Onn][Dif]
│─────────────│
│Ref: 50%     │
│[━━━━━○━━━━] │
│[👁 Toggle]  │
└─────────────┘
```

---

**Layout**: B (Floating Panels)
**Component**: `components/compare/ComparePanel.tsx`
**Key Difference**: Floating, resizable panel with drag support
