# UI Layout Variations

**Audience**: Designers, Developers, Product Managers

## WHAT: Layout Options

This document presents multiple UI layout variations for the Identikit platform, each optimized for different use cases and user preferences.

## WHY: Design Flexibility

Different users have different workflows:
- Law enforcement may need reference image comparison
- Power users prefer maximum canvas space
- Tablet users need touch-friendly interfaces
- New users benefit from familiar layouts

---

## Layout 0: Default (Three-Column)

The primary layout implemented in the application.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  MainToolbar                                                                │
│  [New] [Open] [Save] | [Export]          Untitled Composite*    [Undo][Redo]│
├────────────────┬────────────────────────────────────┬───────────────────────┤
│                │                                    │                       │
│  Feature       │                                    │   Adjustments         │
│  Browser       │                                    │   ─────────────────   │
│  ────────────  │                                    │   Position  X: 400    │
│  [Search...]   │                                    │             Y: 500    │
│                │                                    │   Scale     W: 1.0    │
│  Categories:   │        CompositeCanvas             │             H: 1.0    │
│  ┌──┬──┬──┐    │        (Fabric.js)                 │   Rotation  [====] 0° │
│  │Fa│Ha│Ey│    │                                    │   Opacity   [====]100%│
│  └──┴──┴──┘    │     ┌─────────────────────┐        │   [Flip H] [Flip V]   │
│  ┌──┬──┬──┐    │     │                     │        │                       │
│  │Br│No│Mo│    │     │    Composite        │        ├───────────────────────┤
│  └──┴──┴──┘    │     │    Preview          │        │   Layers              │
│  ┌──┬──┬──┐    │     │                     │        │   ─────────────────   │
│  │Ch│Ea│Ac│    │     │    [Face]           │        │   ☰ ◉ Hair         🗑 │
│  └──┴──┴──┘    │     │    [Eyes]           │        │   ☰ ◉ Eyes         🗑 │
│                │     │    [Nose]           │        │   ☰ ◉ Face Shape   🗑 │
│  [Feature      │     │                     │        │   ☰ ◎ Glasses      🗑 │
│   Grid]        │     └─────────────────────┘        │                       │
│  ┌──┬──┬──┐    │                                    │   ◉ = visible         │
│  │  │  │  │    │                                    │   ◎ = hidden          │
│  ├──┼──┼──┤    │                                    │   🔒 = locked         │
│  │  │  │  │    │                                    │                       │
│  └──┴──┴──┘    │                                    │                       │
│                │                                    │                       │
├────────────────┴────────────────────────────────────┴───────────────────────┤
│  StatusBar     Untitled Composite*    800 x 1000    3 layers    [-][100%][+]│
└─────────────────────────────────────────────────────────────────────────────┘
```

### Specifications

| Panel | Width | Purpose |
|-------|-------|---------|
| Left Sidebar | 288px | Feature library with categories and search |
| Center Canvas | Flexible | Fabric.js compositing canvas with zoom/pan |
| Right Panel | 288px | Adjustment controls (top) + Layer manager (bottom) |
| Top Toolbar | Full | File ops, document name, undo/redo |
| Bottom Status | Full | Document info, selection, zoom controls |

### Best For
- General use
- First-time users
- Standard desktop workflows

---

## Layout A: Horizontal Feature Bar (Photoshop-style)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  [File ▾] [Edit ▾] [View ▾] [Export ▾]              [Undo] [Redo]  [⚙]     │
├─────────────────────────────────────────────────────────────────────────────┤
│  Feature Categories (horizontal scroll)                                      │
│  [Face] [Hair] [Eyes] [Brows] [Nose] [Mouth] [Chin] [Ears] [Accessories]    │
├─────────────────────────────────────────────────────────────────────────────┤
│  Feature Thumbnails (horizontal scroll)                                      │
│  ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐    │
│  │    │ │    │ │    │ │    │ │    │ │    │ │    │ │    │ │    │ │    │ ►  │
│  └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘    │
├───────────────────────────────────────────────────┬─────────────────────────┤
│                                                   │  Properties             │
│                                                   │  ───────────            │
│                                                   │  Position   X ─── Y ─── │
│                                                   │  Size       W ─── H ─── │
│            ┌─────────────────────────┐            │  Rotate     [=======]   │
│            │                         │            │  Opacity    [=======]   │
│            │                         │            │  [Flip H] [Flip V]      │
│            │      CANVAS             │            ├─────────────────────────┤
│            │                         │            │  Layers                 │
│            │                         │            │  ───────────            │
│            │                         │            │  ▼ Group 1              │
│            └─────────────────────────┘            │    ├─ Hair              │
│                                                   │    ├─ Face              │
│                                                   │    └─ Eyes              │
│                                                   │  ▼ Group 2              │
│                                                   │    └─ Glasses           │
├───────────────────────────────────────────────────┴─────────────────────────┤
│  Ready                           800 × 1000       Zoom: 100%   Layers: 4    │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Specifications

| Panel | Position | Height/Width |
|-------|----------|--------------|
| Menu Bar | Top | 40px |
| Category Bar | Top | 48px |
| Feature Strip | Top | 80px |
| Canvas | Center | Flexible |
| Properties | Right | 280px |
| Status Bar | Bottom | 32px |

### Best For
- Users who prefer more vertical canvas space
- Horizontal workflow preferences
- Wide monitors (ultrawide displays)

---

## Layout B: Minimal Floating Panels

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│   ┌─────────────┐                                                           │
│   │ Tools       │                                                           │
│   ├─────────────┤                                                           │
│   │ [Select]    │                                                           │
│   │ [Move]      │         ┌─────────────────────────┐                       │
│   │ [Rotate]    │         │                         │     ┌───────────────┐ │
│   │ [Scale]     │         │                         │     │ Adjustments ✕ │ │
│   │ [Flip]      │         │                         │     ├───────────────┤ │
│   └─────────────┘         │       CANVAS            │     │ X: 400        │ │
│                           │                         │     │ Y: 500        │ │
│   ┌─────────────┐         │                         │     │ Scale: 100%   │ │
│   │ Features  ✕ │         │                         │     │ Rotate: 0°    │ │
│   ├─────────────┤         └─────────────────────────┘     │ Opacity: 100% │ │
│   │ [🔍 Search] │                                         └───────────────┘ │
│   │ ┌──┬──┬──┐  │                                                           │
│   │ │  │  │  │  │                                                           │
│   │ ├──┼──┼──┤  │         ┌───────────────────────────────────────────────┐ │
│   │ │  │  │  │  │         │ Layers                                      ✕ │ │
│   │ ├──┼──┼──┤  │         │ [Hair] [Eyes] [Face] [Nose] [Mouth] [Glasses] │ │
│   │ │  │  │  │  │         └───────────────────────────────────────────────┘ │
│   └─────────────┘                                                           │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  [+][-] 100%                              Untitled.idkit    [New][Save][Exp]│
└─────────────────────────────────────────────────────────────────────────────┘
```

### Specifications

| Panel | Type | Default Position |
|-------|------|------------------|
| Tools | Floating, dockable | Top-left |
| Features | Floating, resizable | Left |
| Adjustments | Floating, collapsible | Top-right |
| Layers | Floating, horizontal | Bottom-center |
| Status Bar | Fixed | Bottom |

### Features
- All panels are draggable and repositionable
- Panels can be collapsed or closed
- Panels remember position between sessions
- Double-click title bar to collapse

### Best For
- Maximum canvas visibility
- Experienced/professional users
- Large monitors
- Custom workflow arrangements

---

## Layout C: Bottom Feature Dock (Touch-Friendly)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  Identikit                    Untitled*              [↩][↪] [💾] [📤] [⚙]  │
├────────────────────────────────────────────────────────────┬────────────────┤
│                                                            │ Selected:      │
│                                                            │ "Left Eye"     │
│                                                            │ ────────────── │
│                                                            │                │
│                                                            │ ○───────● Pos  │
│               ┌───────────────────────────┐                │ ○───────● Size │
│               │                           │                │ ○───────● Rot  │
│               │                           │                │ ○───────● Opa  │
│               │                           │                │                │
│               │        CANVAS             │                │ [↔] [↕] [🔄]   │
│               │                           │                │                │
│               │                           │                ├────────────────┤
│               │                           │                │ LAYERS    [+]  │
│               │                           │                │ ┌────────────┐ │
│               └───────────────────────────┘                │ │◉ Hair     ▲│ │
│                                                            │ │◉ Eyes     ││ │
│                                                            │ │◉ Face     ││ │
│                                                            │ │◎ Glasses  ▼│ │
│                                                            │ └────────────┘ │
├────────────────────────────────────────────────────────────┴────────────────┤
│  Feature Dock                                                    [△ Expand] │
│  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐   │
│  │ Face │ │ Hair │ │ Eyes │ │ Nose │ │Mouth │ │ Ears │ │ Chin │ │ Acc. │   │
│  └──────┘ └──────┘ └──────┘ └──────┘ └──────┘ └──────┘ └──────┘ └──────┘   │
├─────────────────────────────────────────────────────────────────────────────┤
│ ◀ │ ┌──┐ ┌──┐ ┌──┐ ┌──┐ ┌──┐ ┌──┐ ┌──┐ ┌──┐ ┌──┐ ┌──┐ ┌──┐ ┌──┐ │ ▶     │
│   │ └──┘ └──┘ └──┘ └──┘ └──┘ └──┘ └──┘ └──┘ └──┘ └──┘ └──┘ └──┘ │        │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Specifications

| Panel | Position | Behavior |
|-------|----------|----------|
| Header | Top | Fixed, minimal |
| Canvas | Center-left | Touch-optimized |
| Properties | Right | Slim, scrollable |
| Category Dock | Bottom | Expandable |
| Feature Strip | Bottom | Swipeable |

### Touch Gestures
- Pinch to zoom canvas
- Two-finger rotate
- Swipe feature strip
- Tap category to switch
- Long-press for context menu

### Best For
- Tablet/touch interfaces
- Vertical/portrait monitors
- Mobile-first design
- Field use (law enforcement tablets)

---

## Layout D: Split View (Reference Mode)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  [File] [Edit] [View]                              [Undo] [Redo] [Compare]  │
├──────────────────────────────────┬──────────────────────────────────────────┤
│         REFERENCE IMAGE          │            COMPOSITE CANVAS              │
│  ┌────────────────────────────┐  │  ┌────────────────────────────────────┐  │
│  │                            │  │  │                                    │  │
│  │                            │  │  │                                    │  │
│  │     [Upload Reference]     │  │  │                                    │  │
│  │                            │  │  │                                    │  │
│  │     or drag & drop         │  │  │         Working Canvas             │  │
│  │                            │  │  │                                    │  │
│  │                            │  │  │                                    │  │
│  └────────────────────────────┘  │  └────────────────────────────────────┘  │
│  Opacity: [===========] 50%      │  Zoom: [─────●─────] 100%                │
├──────────────────────────────────┴──────────────────────────────────────────┤
│  Categories: [Face][Hair][Eyes][Brows][Nose][Mouth][Chin][Ears][Acc]        │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐  │ Adjustments   │
│  │    │ │    │ │    │ │    │ │    │ │    │ │    │ │    │  │ X:─── Y:───   │
│  └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘  │ [↔][↕][⟳][◐]  │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Specifications

| Panel | Position | Purpose |
|-------|----------|---------|
| Reference View | Left 50% | Display witness photo/sketch |
| Composite Canvas | Right 50% | Working area |
| Category Bar | Bottom | Category selection |
| Feature Strip | Bottom-left | Feature thumbnails |
| Adjustments | Bottom-right | Quick controls |

### Reference Features
- Upload witness photo or sketch
- Overlay on canvas with adjustable opacity
- Lock/unlock for tracing
- Zoom sync between views
- Side-by-side comparison mode

### Best For
- Law enforcement workflow
- Working from witness descriptions
- Photo-to-composite matching
- Training and education

---

## Comparison Matrix

| Layout | Canvas Space | Learning Curve | Touch Support | Best Monitor |
|--------|--------------|----------------|---------------|--------------|
| **Default** | Medium | Low | Basic | Standard 16:9 |
| **A - Horizontal** | High | Medium | Limited | Ultrawide |
| **B - Floating** | Maximum | High | None | Large 27"+ |
| **C - Bottom Dock** | Medium | Low | Excellent | Tablet/Portrait |
| **D - Split View** | Medium | Low | Basic | Dual/Wide |

---

## Implementation Priority

1. **Default (Layout 0)** - Currently implemented
2. **Layout D (Split View)** - High value for law enforcement use case
3. **Layout C (Bottom Dock)** - Tablet support expansion
4. **Layout A (Horizontal)** - Power user option
5. **Layout B (Floating)** - Professional customization

---

## Related

- [Architecture](architecture.md)
- [Developer Guide](../4-development/developer-guide.md)

---

**Last Updated**: 2025-01-01
**Version**: 1.0.0
