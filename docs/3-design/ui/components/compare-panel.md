# Component: Compare Panel

## Compare Mode Selector

```
┌────────────────────────────────────────┐
│ Compare Mode                       [×] │
├────────────────────────────────────────┤
│                                        │
│ Reference Image                        │
│ ─────────────────────────────────────  │
│ ┌─────────────────┐                    │
│ │                 │   witness_photo.jpg│
│ │   [Reference    │   1920 × 2560      │
│ │    Thumbnail]   │                    │
│ │                 │   [Change] [Remove]│
│ └─────────────────┘                    │
│                                        │
│ ─────────────────────────────────────  │
│                                        │
│ View Mode                              │
│ ┌───────┐ ┌───────┐ ┌───────┐         │
│ │ ▌│▐   │ │ ░░░░  │ │ ◀━━▶  │         │
│ │       │ │ ░░░░  │ │       │         │
│ │ Split │ │Overlay│ │Slider │         │
│ │   ●   │ │       │ │       │         │
│ └───────┘ └───────┘ └───────┘         │
│                                        │
│ ┌───────┐ ┌───────┐                    │
│ │ ◐◐◐◐  │ │ ▓▓░░  │                    │
│ │ ◐◐◐◐  │ │ ░░▓▓  │                    │
│ │ Onion │ │ Diff  │                    │
│ │       │ │       │                    │
│ └───────┘ └───────┘                    │
│                                        │
└────────────────────────────────────────┘
```

## Split Mode Controls

```
┌────────────────────────────────────────┐
│ Split View                             │
├────────────────────────────────────────┤
│                                        │
│ Orientation                            │
│ [● Vertical] [○ Horizontal]            │
│                                        │
│ Split Position                         │
│ [━━━━━━━━━○━━━━━━━━━] 50%             │
│                                        │
│ Swap Sides                             │
│ [⇄ Swap Reference/Composite]           │
│                                        │
│ ─────────────────────────────────────  │
│                                        │
│ Labels                                 │
│ [☑] Show labels                        │
│ [☐] Show divider line                  │
│                                        │
│ Sync                                   │
│ [☑] Sync zoom                          │
│ [☑] Sync pan                           │
│                                        │
└────────────────────────────────────────┘
```

## Overlay Mode Controls

```
┌────────────────────────────────────────┐
│ Overlay View                           │
├────────────────────────────────────────┤
│                                        │
│ Reference Opacity                      │
│ [━━━━━━━━━○━━━━━━━━━] 50%             │
│                                        │
│ Blend Mode                             │
│ [Normal                           ▾]   │
│ ┌─────────────────────────────────┐    │
│ │ Normal                          │    │
│ │ Multiply                        │    │
│ │ Screen                          │    │
│ │ Overlay                         │    │
│ │ Difference                      │    │
│ │ Color                           │    │
│ └─────────────────────────────────┘    │
│                                        │
│ Quick Opacity                          │
│ [0%] [25%] [50%] [75%] [100%]         │
│                                        │
│ Toggle                                 │
│ [👁 Show/Hide Reference]    Hotkey: R  │
│                                        │
└────────────────────────────────────────┘
```

## Slider Mode Controls

```
┌────────────────────────────────────────┐
│ Slider View                            │
├────────────────────────────────────────┤
│                                        │
│ Orientation                            │
│ [● Vertical] [○ Horizontal]            │
│                                        │
│ Slider Style                           │
│ [● Line] [○ Spotlight]                 │
│                                        │
│ Line Color                             │
│ [■ #FFFFFF] White                      │
│                                        │
│ Line Width                             │
│ [━━━━○━━━━━━━━━━━━━━] 2px             │
│                                        │
│ ─────────────────────────────────────  │
│                                        │
│ Interaction                            │
│ [☑] Auto-hide slider handle            │
│ [☐] Animate on load                    │
│                                        │
│ Keyboard                               │
│ ← → arrows move slider                 │
│                                        │
└────────────────────────────────────────┘
```

## Onion Skin Controls

```
┌────────────────────────────────────────┐
│ Onion Skin View                        │
├────────────────────────────────────────┤
│                                        │
│ Reference Opacity                      │
│ [━━━━━━━━━○━━━━━━━━━] 30%             │
│                                        │
│ Reference Tint                         │
│ [☑] Apply color tint                   │
│ [■ #00FF00] Green                      │
│                                        │
│ Composite Opacity                      │
│ [━━━━━━━━━━━━━○━━━━━] 70%             │
│                                        │
│ Composite Tint                         │
│ [☑] Apply color tint                   │
│ [■ #FF0000] Red                        │
│                                        │
│ ─────────────────────────────────────  │
│                                        │
│ Presets                                │
│ [Red/Blue] [Green/Red] [Custom]        │
│                                        │
│ [☐] Invert tints                       │
│                                        │
└────────────────────────────────────────┘
```

## Difference Mode Controls

```
┌────────────────────────────────────────┐
│ Difference View                        │
├────────────────────────────────────────┤
│                                        │
│ Mode                                   │
│ [● Absolute] [○ Squared]               │
│                                        │
│ Amplify                                │
│ [━━━━━━━━━○━━━━━━━━━] 1.0x            │
│                                        │
│ Threshold                              │
│ [━━━○━━━━━━━━━━━━━━━] 5%              │
│                                        │
│ ─────────────────────────────────────  │
│                                        │
│ Color Map                              │
│ [● Grayscale]                          │
│ [○ Heat map (cold → hot)]              │
│ [○ Highlight only]                     │
│                                        │
│ Legend                                 │
│ ░░░░░▒▒▒▒▓▓▓▓████                     │
│ 0%   25%  50%  75% 100%               │
│ (no difference → high difference)      │
│                                        │
└────────────────────────────────────────┘
```

## No Reference Image State

```
┌────────────────────────────────────────┐
│ Compare Mode                       [×] │
├────────────────────────────────────────┤
│                                        │
│                                        │
│            📷                          │
│                                        │
│    No reference image                  │
│                                        │
│    Add a reference image to            │
│    compare with your composite         │
│                                        │
│    [📤 Upload Image]                   │
│                                        │
│    or drag & drop an image here        │
│                                        │
│                                        │
└────────────────────────────────────────┘
```

## Compare Statistics

```
┌────────────────────────────────────────┐
│ Comparison Stats                       │
├────────────────────────────────────────┤
│                                        │
│ Similarity Score                       │
│ ─────────────────────────────────────  │
│                                        │
│   ████████████░░░░░░░  72%            │
│                                        │
│ Breakdown                              │
│ • Position alignment:   85%            │
│ • Proportion match:     78%            │
│ • Feature coverage:     68%            │
│ • Outline similarity:   62%            │
│                                        │
│ Note: Score is approximate and         │
│ should not replace visual comparison   │
│                                        │
│ [📊 Export Report]                     │
│                                        │
└────────────────────────────────────────┘
```

## Floating Compare Controls

```
        ┌─────────────────────────────────┐
        │ [◀] [▐│▌] [◐] [◀━▶] [▓░] [▶]   │
        │      ↑                          │
        │   Current mode                  │
        │                                 │
        │ Opacity: [━━━━━○━━━] 50%       │
        │                                 │
        │ [👁 Toggle] [⚙ Options]        │
        └─────────────────────────────────┘
```

## Keyboard Shortcuts

```
┌────────────────────────────────────────┐
│ Compare Shortcuts                      │
├────────────────────────────────────────┤
│                                        │
│ C          Toggle compare mode         │
│ 1          Split view                  │
│ 2          Overlay view                │
│ 3          Slider view                 │
│ 4          Onion skin view             │
│ 5          Difference view             │
│                                        │
│ R          Toggle reference visibility │
│ [ ]        Decrease/increase opacity   │
│ Space      Sync zoom (hold)            │
│ Esc        Exit compare mode           │
│                                        │
└────────────────────────────────────────┘
```

---

**Component**: `components/compare/ComparePanel.tsx`
**Props**: `referenceImage`, `compareMode`, `settings`, `onModeChange`, `onSettingsChange`
