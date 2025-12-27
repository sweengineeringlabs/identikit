# Component: Feature Dock (Layout C - Bottom Dock)

## Expanded Dock State

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  Features [▼]    [🔍 Search...] │ Hair │ Eyes● │ Nose │ Mouth │ Brows │ Ears │ More ▾  │
│  ┌──────────────────────────────────────────────────────────────────────────────────┐   │
│  │ ◄ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐► │   │
│  │   │ 👁️ │ │ 👁️ │ │ 👁️ │ │ 👁️ │ │ 👁️ │ │ 👁️ │ │ 👁️ │ │ 👁️ │ │ 👁️ │ │ 👁️ │ │ 👁️ │  │   │
│  │   │👁️  │ │👁️  │ │👁️  │ │👁️  │ │👁️  │ │👁️  │ │👁️  │ │👁️  │ │👁️  │ │👁️  │ │👁️  │  │   │
│  │   └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘  │   │
│  │   Almond● Round  Asian  Hooded Wide   Deep   Close  Mono.. Down.. Uptur Sleepy  │   │
│  └──────────────────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Collapsed Dock State

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  Features [▲]    [🔍 Search...] │ Hair │ Eyes │ Nose │ Mouth │ Brows │ Ears │ More ▾   │
└─────────────────────────────────────────────────────────────────────────────────────────┘
       ↑
   Click to expand
```

## Two-Row Expanded

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  Features [▼]    [🔍 Search...] │ Hair● │ Eyes │ Nose │ Mouth │ Brows │ Ears │ More ▾  │
│  ┌──────────────────────────────────────────────────────────────────────────────────┐   │
│  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐    │   │
│  │  │        │ │        │ │        │ │        │ │        │ │        │ │        │    │   │
│  │  │  Long  │ │  Short │ │  Wavy  │ │ Curly  │ │Straight│ │  Bald  │ │ Mohawk │    │   │
│  │  │        │ │        │ │        │ │        │ │        │ │        │ │        │    │   │
│  │  └────────┘ └────────┘ └────────┘ └────────┘ └────────┘ └────────┘ └────────┘    │   │
│  │  Long-001   Short-001  Wavy-001   Curly-001  Straight   Bald-001   Mohawk-01     │   │
│  │                                                                                   │   │
│  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐    │   │
│  │  │        │ │        │ │        │ │        │ │        │ │        │ │        │    │   │
│  │  │ Ponytl │ │  Bun   │ │ Braids │ │ Afro   │ │ Fade   │ │  Buzz  │ │ Pixie  │    │   │
│  │  │        │ │        │ │        │ │        │ │        │ │        │ │        │    │   │
│  │  └────────┘ └────────┘ └────────┘ └────────┘ └────────┘ └────────┘ └────────┘    │   │
│  │  Ponytail   Bun-001    Braids-01  Afro-001   Fade-001   Buzz-001   Pixie-001     │   │
│  │                                                                              ▼   │   │
│  └──────────────────────────────────────────────────────────────────────────────────┘   │
│  Showing 14 of 24 features                                                  [View All]  │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Category Tabs

```
│ Hair │ Eyes● │ Nose │ Mouth │ Brows │ Ears │ More ▾  │
   │      ↑                               │       │
   │  Active tab                          │       └─ Overflow menu
   │  (highlighted)                       │
   └──────────────────────────────────────┴─ Inactive tabs
```

## Category Overflow Menu

```
                                                    │ More ▾  │
                                                          │
                                                          ▼
                                                   ┌────────────┐
                                                   │ Chin       │
                                                   │ Face Shape │
                                                   │ Accessories│
                                                   │ ────────── │
                                                   │ View All   │
                                                   └────────────┘
```

## Feature Card States

### Default
```
┌────────┐
│        │
│  👁️👁️   │
│        │
└────────┘
 Almond
```

### Hover
```
╔════════╗
║        ║
║  👁️👁️   ║ ← Blue border
║        ║
╚════════╝
 Almond
[+ Add]
```

### Selected (In Use)
```
┌────────┐
│░░░░░░░░│ ← Highlighted background
│░ 👁️👁️ ░│
│░░░░░░░░│
└────────┘
 Almond ●
    ↑
 In use indicator
```

## Search Active

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  Features [▼]    [🔍 round                    ✕] │ All Results                          │
│  ┌──────────────────────────────────────────────────────────────────────────────────┐   │
│  │  Search results for "round" (8 found)                                             │   │
│  │                                                                                   │   │
│  │  Eyes (3)                                                                         │   │
│  │  ┌────────┐ ┌────────┐ ┌────────┐                                                │   │
│  │  │   👁️👁️  │ │   👁️👁️  │ │   👁️👁️  │                                                │   │
│  │  └────────┘ └────────┘ └────────┘                                                │   │
│  │  Round-001  Round-002  Round-Big                                                  │   │
│  │                                                                                   │   │
│  │  Face Shape (3)        Nose (2)                                                   │   │
│  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐                          │   │
│  │  │   ○    │ │   ○    │ │   ○    │ │   👃   │ │   👃   │                          │   │
│  │  └────────┘ └────────┘ └────────┘ └────────┘ └────────┘                          │   │
│  │  Round      Round-Wide Round-Soft Round-Tip  Round-Bulb                           │   │
│  └──────────────────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Horizontal Scroll Indicators

```
◄ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ►
│ │    │ │    │ │    │ │    │ │    │ │    │ │    │ │    │ │
│ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ │
│                                                         │
└─ Scroll left button                   Scroll right ─────┘
   (disabled at start)                  (hidden at end)
```

## Empty Category

```
┌──────────────────────────────────────────────────────────────────────────────────────┐
│                                                                                      │
│                                       📦                                             │
│                                                                                      │
│                               No features in this category                           │
│                                                                                      │
│                              Coming in a future update                               │
│                                                                                      │
└──────────────────────────────────────────────────────────────────────────────────────┘
```

## Drag to Canvas

```
                     ┌────────┐
                     │  👁️👁️   │ ← Dragging feature
                     └────────┘
                        │
                        │  (drag indicator line)
                        │
                        ▼
┌────────────────────────────────────────────────────────────┐
│                                                            │
│                         CANVAS                             │
│                    [Drop zone highlight]                   │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

## Actions

| Element | Action |
|---------|--------|
| [▼]/[▲] | Toggle dock expansion |
| Category tabs | Switch feature category |
| Search input | Filter features |
| Feature card click | Add to canvas |
| Feature card drag | Drag to canvas |
| ◄/► arrows | Scroll horizontally |
| [View All] | Open full-screen browser |
| [More ▾] | Show overflow categories |

---

**Layout**: C (Bottom Dock)
**Component**: `components/feature-library/FeatureDock.tsx`
**Key Difference**: Horizontal layout with category tabs and horizontal scroll
