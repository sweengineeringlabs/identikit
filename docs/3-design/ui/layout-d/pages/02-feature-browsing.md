# Screen: Feature Browsing (Layout D - Split View Focus)

## Full Feature Browser

```
┌────────────────────────────────────────┬────────────────────────────────────────────────┐
│                                        │                                                │
│  [🔍 Search all features...]           │                                                │
│                                        │                                                │
│  ──────────────────────────────────────│                                                │
│                                        │                                                │
│  Categories                            │                                                │
│                                        │                                                │
│  ▾ Hair (24)                           │                                                │
│  ┌────┐ ┌────┐ ┌────┐ ┌────┐          │                                                │
│  │    │ │    │ │    │ │    │          │                                                │
│  │Long│ │Shrt│ │Wavy│ │Curl│          │                 CANVAS                         │
│  └────┘ └────┘ └────┘ └────┘          │                                                │
│  ┌────┐ ┌────┐ ┌────┐ ┌────┐          │                                                │
│  │    │ │    │ │    │ │    │          │                                                │
│  │Bald│ │Afro│ │Bun │ │Pixe│          │                                                │
│  └────┘ └────┘ └────┘ └────┘          │                                                │
│                                        │                                                │
│  ▾ Eyes (32)                           │                                                │
│  ┌────┐ ┌────┐ ┌────┐ ┌────┐          │                                                │
│  │ 👁️ │ │ 👁️ │ │ 👁️ │ │ 👁️ │          │                                                │
│  │👁️  │ │👁️  │ │👁️  │ │👁️  │          │                                                │
│  └────┘ └────┘ └────┘ └────┘          │                                                │
│  Almond Round  Asian  Hooded          │                                                │
│                                        │                                                │
│  ▸ Nose (20)                           │                                                │
│  ▸ Mouth (16)                          │                                                │
│  ▸ Eyebrows (18)                       │                                                │
│  ▸ Ears (12)                           │                                                │
│  ▸ Chin (14)                           │                                                │
│  ▸ Face Shape (10)                     │                                                │
│  ▸ Accessories (28)                    │                                                │
│                                        │                                                │
└────────────────────────────────────────┴────────────────────────────────────────────────┘
```

## Search Active

```
┌────────────────────────────────────────┬────────────────────────────────────────────────┐
│                                        │                                                │
│  [🔍 round                         ✕]  │                                                │
│                                        │                                                │
│  ──────────────────────────────────────│                                                │
│                                        │                                                │
│  Search Results (8)                    │                                                │
│                                        │                                                │
│  Eyes (3)                              │                                                │
│  ┌────┐ ┌────┐ ┌────┐                 │                                                │
│  │ 👁️ │ │ 👁️ │ │ 👁️ │                 │                                                │
│  │👁️  │ │👁️  │ │👁️  │                 │                                                │
│  └────┘ └────┘ └────┘                 │                 CANVAS                         │
│  Round  Round2 Round-Lg               │                                                │
│                                        │                                                │
│  Face Shape (3)                        │                                                │
│  ┌────┐ ┌────┐ ┌────┐                 │                                                │
│  │  ○ │ │  ○ │ │  ○ │                 │                                                │
│  └────┘ └────┘ └────┘                 │                                                │
│  Round  Round-W Round-S               │                                                │
│                                        │                                                │
│  Nose (2)                              │                                                │
│  ┌────┐ ┌────┐                        │                                                │
│  │ 👃 │ │ 👃 │                        │                                                │
│  └────┘ └────┘                        │                                                │
│  Round  Round-B                       │                                                │
│                                        │                                                │
│  [Clear Search]                        │                                                │
│                                        │                                                │
└────────────────────────────────────────┴────────────────────────────────────────────────┘
```

## Feature Hover Detail

```
┌────┐
│    │
│Long│ ← Hovering
└────┘
   │
   ▼
┌────────────────────────────────┐
│  Long Wavy Hair                │
│  hair-long-wavy-001            │
│                                │
│  Long hairstyle with natural   │
│  wave pattern.                 │
│                                │
│  [Add to Canvas]  [Details]    │
└────────────────────────────────┘
```

## Category Accordion States

```
▾ Hair (24)           ← Expanded (showing features)
  ┌────┐ ┌────┐ ...

▸ Eyes (32)           ← Collapsed (click to expand)

▸ Nose (20)           ← Collapsed
```

## Grid Sizing Options

```
View: [▦ Small] [▦▦ Medium●] [▦▦▦ Large]

Small:   6 per row
Medium:  4 per row (default)
Large:   3 per row (detailed)
```

## Drag to Canvas

```
┌────────────────────────────────────────┬────────────────────────────────────────────────┐
│                                        │                                                │
│  ▾ Eyes (32)                           │                                                │
│  ┌────┐ ┌────┐ ┌────┐ ┌────┐          │              ╔════════════════════════════════╗│
│  │    │ │    │ │ 👁️ │ │    │ ←Dragging│              ║                                ║│
│  │    │ │    │ │👁️  │ │    │          │              ║     Drop here to add           ║│
│  └────┘ └────┘ └──│─┘ └────┘          │              ║                                ║│
│                   │                    │              ╚════════════════════════════════╝│
│                   │                    │                         │                      │
│                   └────────────────────│─────────────────────────┘                      │
│                      Drag indicator    │               Drop zone highlight              │
│                                        │                                                │
└────────────────────────────────────────┴────────────────────────────────────────────────┘
```

## Actions

| Element | Action |
|---------|--------|
| Search input | Filter all features |
| Category header | Expand/collapse |
| Feature card click | Add to canvas |
| Feature card drag | Drag to canvas |
| Feature card hover | Show quick info |
| [Details] | Open detail modal |
| View toggle | Change grid density |
| [Clear Search] | Reset search |

---

**Layout**: D (Split View Focus)
**Component**: `FeatureBrowser.tsx`
**Key Difference**: Full-height browser with accordion categories
