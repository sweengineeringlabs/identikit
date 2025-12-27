# Component: Adjustment Panel (Layout A - Horizontal Split)

## Default State (No Selection)

```
┌────────────────────────────────┐
│ [Adjustments ●] [Layers]       │
├────────────────────────────────┤
│                                │
│                                │
│          ◇                     │
│                                │
│   No layer selected            │
│                                │
│   Select a layer from the      │
│   canvas or layer panel        │
│                                │
│                                │
└────────────────────────────────┘
```

## Layer Selected

```
┌────────────────────────────────┐
│ [Adjustments ●] [Layers]       │
├────────────────────────────────┤
│                                │
│ 👁️ Eyes - Almond Style 1        │
│ ────────────────────────────   │
│                                │
│ Position                       │
│ ────────────────────────────   │
│ X: [____320___] px             │
│ Y: [____450___] px             │
│ [⌖ Center]                     │
│                                │
│ Size                           │
│ ────────────────────────────   │
│ W: [____180___] px  [🔗]       │
│ H: [_____60___] px             │
│                                │
│ Scale                          │
│ ────────────────────────────   │
│ [━━━━━━━━━○━━] 100%            │
│                                │
│ Rotation                       │
│ ────────────────────────────   │
│ [____0____]°  [↻ 90] [↺ 90]    │
│                                │
│ Flip                           │
│ ────────────────────────────   │
│ [↔ Horizontal] [↕ Vertical]    │
│                                │
│ Opacity                        │
│ ────────────────────────────   │
│ [━━━━━━━━━━━━━━━○] 100%        │
│                                │
│ ────────────────────────────   │
│ [🔄 Reset All]                 │
│                                │
└────────────────────────────────┘
```

## With Color Zones

```
┌────────────────────────────────┐
│ [Adjustments ●] [Layers]       │
├────────────────────────────────┤
│                                │
│ 👁️ Eyes - Almond Style 1        │
│ ────────────────────────────   │
│                                │
│ ▸ Position      320, 450       │
│ ▸ Size          180 × 60       │
│ ▸ Transform                    │
│ ▾ Color Zones                  │
│   ──────────────────────────   │
│                                │
│   Iris                         │
│   [■ #4A6B3D ▾] Green          │
│   Presets: [■][■][■][■][■]     │
│                                │
│   Sclera                       │
│   [■ #FFFFFF ▾] White          │
│                                │
│   Outline                      │
│   [■ #333333 ▾] Dark           │
│                                │
│   [🔄 Reset Colors]            │
│                                │
└────────────────────────────────┘
```

## Multiple Selection

```
┌────────────────────────────────┐
│ [Adjustments ●] [Layers]       │
├────────────────────────────────┤
│                                │
│ 3 layers selected              │
│ ────────────────────────────   │
│ • Eyes - Almond                │
│ • Nose - Straight              │
│ • Mouth - Medium               │
│                                │
│ Common Properties              │
│ ────────────────────────────   │
│                                │
│ Scale (relative)               │
│ [━━━━━━━━━○━━] 100%            │
│                                │
│ Opacity                        │
│ [━━━━━━━━━━━━━━━○] 100%        │
│                                │
│ Align                          │
│ [⫷][⫿][⫸] [⫯][⫰][⫱]          │
│                                │
│ [Group Layers]                 │
│                                │
└────────────────────────────────┘
```

## Locked Layer

```
┌────────────────────────────────┐
│ [Adjustments ●] [Layers]       │
├────────────────────────────────┤
│                                │
│ 🔒 Eyes - Almond Style 1       │
│ ────────────────────────────   │
│                                │
│ ⚠ This layer is locked         │
│                                │
│ [🔓 Unlock Layer]              │
│                                │
│ ────────────────────────────   │
│                                │
│ Position (read-only)           │
│ X: 320 px   Y: 450 px          │
│                                │
│ Size: 180 × 60 px              │
│ Rotation: 0°                   │
│ Opacity: 100%                  │
│                                │
└────────────────────────────────┘
```

---

**Layout**: A (Horizontal Split)
**Component**: `components/adjustment/AdjustmentPanel.tsx`
**Key Difference**: Tabbed interface shared with Layers panel
