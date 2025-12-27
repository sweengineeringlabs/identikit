# Modal: New Composite (Layout D - Split View Focus)

## Default State

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  New Composite                                                              [✕]         │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                         │
│  Composite Name                                                                         │
│  ┌─────────────────────────────────────────────────────────────────────────────────┐   │
│  │ Untitled Composite                                                               │   │
│  └─────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                         │
│  Canvas Size                                                                            │
│  ─────────────────────────────────────────────────────────────                         │
│                                                                                         │
│  Preset: [Portrait Standard                                                    ▾]       │
│                                                                                         │
│  ┌────────────────────┐  ┌────────────────────┐  ┌────────────────────┐                │
│  │    ┌────────┐      │  │    ┌──────────┐    │  │    ┌──────┐        │                │
│  │    │        │      │  │    │          │    │  │    │      │        │                │
│  │    │        │      │  │    │          │    │  │    │      │        │                │
│  │    │        │      │  │    └──────────┘    │  │    └──────┘        │                │
│  │    └────────┘      │  │                    │  │                    │                │
│  │  Portrait (●)      │  │  Landscape         │  │  Square            │                │
│  │  800 × 1000        │  │  1000 × 800        │  │  800 × 800         │                │
│  └────────────────────┘  └────────────────────┘  └────────────────────┘                │
│                                                                                         │
│  Custom Size                                                                            │
│  Width: [______800______] px    Height: [_____1000_____] px    [🔗 Lock]               │
│                                                                                         │
│  Background: [● White] [○ Transparent] [○ Custom: ■ #______ ]                          │
│                                                                                         │
│                                                        [Cancel]   [Create Composite]    │
│                                                                                         │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## With Template Selection

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  New Composite                                                              [✕]         │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                         │
│  [ Blank ● ]  [ Template ]  [ Case Info ]                                              │
│  ─────────────────────────────────────────────────────────────────────────────────────  │
│                                                                                         │
│  Select Template                                                                        │
│                                                                                         │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐    │
│  │     ┌───┐       │  │     ┌───┐       │  │     ┌───┐       │  │                 │    │
│  │     │   │       │  │     │   │       │  │     │   │       │  │       +         │    │
│  │     │ ♂ │       │  │     │ ? │       │  │     │ ♀ │       │  │     Blank       │    │
│  │     └───┘       │  │     └───┘       │  │     └───┘       │  │                 │    │
│  │   Male Base     │  │   Neutral       │  │  Female Base    │  │     Empty       │    │
│  │      (●)        │  │                 │  │                 │  │                 │    │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  └─────────────────┘    │
│                                                                                         │
│  Template includes:                                                                     │
│  • Base face shape (oval)                                                              │
│  • Default ear position                                                                │
│  • Skin tone guidelines                                                                │
│                                                                                         │
│  Composite Name: [______Male_Template_01______]                                        │
│                                                                                         │
│                                                        [Cancel]   [Create Composite]    │
│                                                                                         │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Actions

| Element | Action |
|---------|--------|
| Tab navigation | Switch sections |
| Preset dropdown | Select predefined sizes |
| Orientation cards | Quick size selection |
| Custom size inputs | Manual dimensions |
| [🔗 Lock] | Maintain aspect ratio |
| Background options | Set canvas background |
| Template cards | Select starting template |
| [Create Composite] | Create and open |
| [Cancel] | Close without creating |

---

**Layout**: D (Split View Focus)
**Trigger**: File > New or Ctrl+N
**Component**: `modals/NewCompositeModal.tsx`
