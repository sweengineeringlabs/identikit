# Component: Adjustment Panel (Layout B - Floating Panels)

## Floating Adjustment Panel

```
┌─Adjustments──────────[_][□][✕]┐
│ 👁️ Eyes - Almond Style 1       │
│ ─────────────────────────     │
│                               │
│ Position                      │
│ X: [___320___] px             │
│ Y: [___450___] px             │
│ [⌖ Center]                    │
│                               │
│ Size                          │
│ W: [___180___] px  [🔗]       │
│ H: [____60___] px             │
│                               │
│ Scale                         │
│ [━━━━━━━━━○━━] 100%           │
│                               │
│ Rotation                      │
│ [___0___]°  [↻ 90] [↺ 90]     │
│                               │
│ Flip                          │
│ [↔ Horizontal] [↕ Vertical]   │
│                               │
│ Opacity                       │
│ [━━━━━━━━━━━━━━━○] 100%       │
│                               │
│ [🔄 Reset All]                │
└───────────────────────────────┘
```

## No Selection State

```
┌─Adjustments──────────[_][□][✕]┐
│                               │
│          ◇                    │
│                               │
│   No layer selected           │
│                               │
│   Select a layer from the     │
│   canvas or Layers panel      │
│                               │
└───────────────────────────────┘
```

## With Color Zones

```
┌─Adjustments──────────[_][□][✕]┐
│ 👁️ Eyes - Almond Style 1       │
│ ─────────────────────────     │
│                               │
│ ▸ Position      320, 450      │
│ ▸ Size          180 × 60      │
│ ▸ Transform                   │
│ ▾ Color Zones                 │
│   ────────────────────────    │
│                               │
│   Iris                        │
│   [■ #4A6B3D ▾] Green         │
│   [■][■][■][■][■]             │
│                               │
│   Sclera                      │
│   [■ #FFFFFF ▾] White         │
│                               │
│   Outline                     │
│   [■ #333333 ▾] Dark          │
│                               │
│   [🔄 Reset Colors]           │
└───────────────────────────────┘
```

## Compact Mode

```
┌─Adjust──[✕]┐
│Eyes-Almond │
│────────────│
│X:[320] Y:[450]
│Scale: 100% │
│Rot: 0°     │
│Opacity:100%│
│[🔄 Reset]  │
└────────────┘
```

---

**Layout**: B (Floating Panels)
**Component**: `components/adjustment/AdjustmentPanel.tsx`
**Key Difference**: Panel is floating, resizable; adapts to size
