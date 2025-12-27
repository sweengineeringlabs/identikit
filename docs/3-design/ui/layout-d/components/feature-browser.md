# Component: Feature Browser (Layout D - Split View Focus)

## Full-Height Browser

```
┌────────────────────────────────────────┐
│                                        │
│  [🔍 Search all features...]           │
│                                        │
│  ──────────────────────────────────────│
│                                        │
│  Categories                            │
│                                        │
│  ▾ Hair (24)                           │
│  ┌────┐ ┌────┐ ┌────┐ ┌────┐          │
│  │    │ │    │ │    │ │    │          │
│  │Long│ │Shrt│ │Wavy│ │Curl│          │
│  └────┘ └────┘ └────┘ └────┘          │
│  ┌────┐ ┌────┐ ┌────┐ ┌────┐          │
│  │    │ │    │ │    │ │    │          │
│  │Bald│ │Afro│ │Bun │ │Pixe│          │
│  └────┘ └────┘ └────┘ └────┘          │
│                                        │
│  ▾ Eyes (32)                           │
│  ┌────┐ ┌────┐ ┌────┐ ┌────┐          │
│  │ 👁️ │ │ 👁️ │ │ 👁️ │ │ 👁️ │          │
│  │👁️  │ │👁️  │ │👁️  │ │👁️  │          │
│  └────┘ └────┘ └────┘ └────┘          │
│  Almond Round  Asian  Hooded          │
│                                        │
│  ▸ Nose (20)                           │
│  ▸ Mouth (16)                          │
│  ▸ Eyebrows (18)                       │
│  ▸ Ears (12)                           │
│  ▸ Chin (14)                           │
│  ▸ Face Shape (10)                     │
│  ▸ Accessories (28)                    │
│                                        │
│  ──────────────────────────────────────│
│                                        │
│  Layers                                │
│  [👁] Hair                             │
│  [👁] Eyes ●                           │
│  [👁] Nose                             │
│  [👁] Mouth                            │
│  ─────────────                         │
│  [+] [📁] [🗑] 4 layers                │
│                                        │
└────────────────────────────────────────┘
```

## Category States

```
▾ Hair (24)              ← Expanded
  [Feature grid...]

▸ Eyes (32)              ← Collapsed (click to expand)
```

## Feature Grid

```
┌────┐ ┌────┐ ┌────┐ ┌────┐
│    │ │    │ │    │ │    │  ← 4 per row (default)
│Long│ │Shrt│ │Wavy│ │Curl│
└────┘ └────┘ └────┘ └────┘
  ↑      ↑      ↑      ↑
Labels below each thumbnail
```

## Feature Card States

### Default
```
┌────┐
│    │
│Long│
└────┘
```

### Hover
```
╔════╗
║    ║ ← Blue border
║Long║
╚════╝
[Add]
```

### In Use
```
┌────┐
│░░░░│ ← Highlighted
│Long│
└────┘●
    ↑
  Dot indicator
```

## Search Mode

```
┌────────────────────────────────────────┐
│                                        │
│  [🔍 round                         ✕]  │
│                                        │
│  ──────────────────────────────────────│
│                                        │
│  Search Results (8)                    │
│                                        │
│  Eyes (3)                              │
│  ┌────┐ ┌────┐ ┌────┐                 │
│  │ 👁️ │ │ 👁️ │ │ 👁️ │                 │
│  └────┘ └────┘ └────┘                 │
│  Round  Round2 Round-Lg               │
│                                        │
│  Face Shape (3)                        │
│  ┌────┐ ┌────┐ ┌────┐                 │
│  │  ○ │ │  ○ │ │  ○ │                 │
│  └────┘ └────┘ └────┘                 │
│  Round  Round-W Round-S               │
│                                        │
│  [Clear Search]                        │
│                                        │
└────────────────────────────────────────┘
```

## Layer List (Integrated)

```
──────────────────────────────────────
Layers
[👁][🔓] Hair
[👁][🔓] Eyes ● ← Selected
[👁][🔓] Nose
[👁][🔓] Mouth
─────────────
[+] [📁] [🗑] 4 layers
```

## Compare Mode State

When in compare mode, browser shows reference management:

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
│  [🔄 Change] [🗑 Remove]               │
│                                        │
│  ──────────────────────────────────────│
│                                        │
│  Compare Settings                      │
│  [Settings for current mode...]        │
│                                        │
│  ──────────────────────────────────────│
│                                        │
│  Layers                                │
│  [👁] Hair                             │
│  [👁] Eyes ●                           │
│  [👁] Nose                             │
│  [👁] Mouth                            │
│                                        │
└────────────────────────────────────────┘
```

## Actions

| Element | Action |
|---------|--------|
| Search input | Filter features |
| Category header | Expand/collapse |
| Feature card click | Add to canvas |
| Feature card drag | Drag to canvas |
| [👁] | Toggle layer visibility |
| [🔓] | Toggle layer lock |
| Layer row click | Select layer |
| [+] | Add new layer |
| [📁] | Group layers |
| [🗑] | Delete selected |

---

**Layout**: D (Split View Focus)
**Component**: `components/feature-library/FeatureBrowser.tsx`
**Key Difference**: Full-height with integrated layer list
