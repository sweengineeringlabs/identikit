# Component: Sidebar (Feature Library)

## Default State - Collapsed Categories

```
┌────────────────────────────┐
│ Feature Library        [×] │
├────────────────────────────┤
│ [🔍 Search features...   ] │
├────────────────────────────┤
│                            │
│  ▸ Hair              (24)  │
│  ▸ Face Shape        (12)  │
│  ▸ Eyes              (32)  │
│  ▸ Eyebrows          (18)  │
│  ▸ Nose              (20)  │
│  ▸ Mouth             (16)  │
│  ▸ Chin               (8)  │
│  ▸ Ears              (10)  │
│  ▸ Accessories       (28)  │
│                            │
├────────────────────────────┤
│ [📁 Custom Features]       │
│ [📥 Import Feature...]     │
└────────────────────────────┘
```

## Expanded Category

```
┌────────────────────────────┐
│ Feature Library        [×] │
├────────────────────────────┤
│ [🔍 Search features...   ] │
├────────────────────────────┤
│                            │
│  ▸ Hair              (24)  │
│  ▾ Eyes              (32)  │
│  ├────────────────────────┤│
│  │ Filter: [All      ▾]   ││
│  │                        ││
│  │ ┌─────┐ ┌─────┐ ┌─────┐││
│  │ │👁️ 👁️ │ │👁️ 👁️ │ │👁️ 👁️ │││
│  │ │     │ │     │ │     │││
│  │ │Almon│ │Round│ │Narro│││
│  │ └─────┘ └─────┘ └─────┘││
│  │                        ││
│  │ ┌─────┐ ┌─────┐ ┌─────┐││
│  │ │👁️ 👁️ │ │👁️ 👁️ │ │👁️ 👁️ │││
│  │ │     │ │     │ │     │││
│  │ │Wide │ │Asian│ │Deep │││
│  │ └─────┘ └─────┘ └─────┘││
│  │                        ││
│  │ [Load More...]         ││
│  └────────────────────────┘│
│  ▸ Eyebrows          (18)  │
│  ▸ Nose              (20)  │
│  ...                       │
└────────────────────────────┘
```

## Feature Item States

### Default
```
┌─────────────┐
│             │
│   [Preview] │
│             │
│ Almond      │
└─────────────┘
```

### Hover
```
┌─────────────┐
│ ┌─────────┐ │
│ │[Preview]│ │  ← Slight scale up
│ └─────────┘ │    Border highlight
│ Almond      │
└─────────────┘
```

### Selected (on canvas)
```
┌─────────────┐
│ ┌─────────┐ │
│ │[Preview]│ │  ← Blue border
│ │    ✓    │ │    Checkmark badge
│ └─────────┘ │
│ Almond  ●   │  ← "In use" indicator
└─────────────┘
```

### Dragging
```
        ┌───────┐
        │[👁️ 👁️] │  ← Ghost preview follows cursor
        │Almond │
        └───────┘
            ↓
   [Drop on canvas]
```

## Search Active

```
┌────────────────────────────┐
│ Feature Library        [×] │
├────────────────────────────┤
│ [🔍 almond               ✕]│
├────────────────────────────┤
│                            │
│ Results for "almond"       │
│                            │
│ Eyes (2)                   │
│ ┌─────┐ ┌─────┐            │
│ │👁️ 👁️ │ │👁️ 👁️ │            │
│ │Almon│ │Almon│            │
│ │  1  │ │  2  │            │
│ └─────┘ └─────┘            │
│                            │
│ Face Shape (1)             │
│ ┌─────┐                    │
│ │     │                    │
│ │Almon│                    │
│ │ Oval│                    │
│ └─────┘                    │
│                            │
│ 3 results                  │
│                            │
└────────────────────────────┘
```

## Category Filter Dropdown

```
┌────────────────────────────┐
│ Filter: [All        ▾    ] │
│         ┌────────────────┐ │
│         │ All            │ │
│         │ ────────────── │ │
│         │ Almond         │ │
│         │ Round          │ │
│         │ Narrow         │ │
│         │ Wide           │ │
│         │ Asian          │ │
│         │ Deep-set       │ │
│         │ Hooded         │ │
│         └────────────────┘ │
└────────────────────────────┘
```

## Feature Detail Popup (on hover/click)

```
┌────────────────────────────┐
│                            │
│  ▾ Eyes              (32)  │
│  ├────────────────────────┤│
│  │ ┌─────┐ ┌─────┐ ┌─────┐││
│  │ │👁️ 👁️ │ │👁️ 👁️ │ │     │││
│  │ └─────┘ └─────┴────────┴┴─────────────┐
│  │         ┌─────────────────────────────┐│
│  │         │ Round Eyes - Style 2        ││
│  │         │ ─────────────────────────── ││
│  │         │ Category: Eyes              ││
│  │         │ ID: eyes-round-002          ││
│  │         │ Tags: round, large, open    ││
│  │         │                             ││
│  │         │ [View Details] [Add ➕]     ││
│  │         └─────────────────────────────┘│
│  │                        ││
└──┴────────────────────────┘┘
```

## Collapsed Sidebar (Icon Only)

```
┌───┐
│ 📚│
├───┤
│ 💇│
│ 😐│
│ 👁️│
│ 🤨│
│ 👃│
│ 👄│
│ 🔽│
│ 👂│
│ 👓│
├───┤
│ ▸ │ ← Expand
└───┘
```

## Custom Features Section

```
┌────────────────────────────┐
│ ▾ Custom Features     (3)  │
│ ├────────────────────────┤ │
│ │                        │ │
│ │ ┌─────┐ ┌─────┐ ┌─────┐│ │
│ │ │     │ │     │ │     ││ │
│ │ │Scar │ │Tatto│ │Birth││ │
│ │ │  1  │ │  1  │ │mark ││ │
│ │ └─────┘ └─────┘ └─────┘│ │
│ │                        │ │
│ │ [+ Import Feature...]  │ │
│ └────────────────────────┘ │
└────────────────────────────┘
```

## Empty State

```
┌────────────────────────────┐
│ Feature Library        [×] │
├────────────────────────────┤
│ [🔍 Search features...   ] │
├────────────────────────────┤
│                            │
│                            │
│       📁                   │
│                            │
│  No features found         │
│                            │
│  Feature assets could not  │
│  be loaded. Check that     │
│  the features folder       │
│  exists at:                │
│                            │
│  /features/                │
│                            │
│  [Reload] [Import...]      │
│                            │
│                            │
└────────────────────────────┘
```

---

**Component**: `components/feature-library/FeatureBrowser.tsx`
**Props**: `categories`, `onFeatureSelect`, `onFeatureDrag`, `selectedFeatures`
