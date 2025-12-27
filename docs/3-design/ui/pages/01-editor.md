# Page 01: Editor (Main Workspace)

## Standard Mode (No Compare)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  [←] [New][Open][Save] | [Export] [📷]    Case-2025-01*     [↩][↪] [⋮]     │
├────────────────┬────────────────────────────────────┬───────────────────────┤
│                │                                    │                       │
│  Features      │                                    │   Adjustments         │
│  ────────────  │                                    │   ─────────────────   │
│  [🔍 Search..] │                                    │   Selected: Eyes      │
│                │                                    │                       │
│  [Fa][Ha][Ey]  │                                    │   Position            │
│  [Br][No][Mo]  │        ┌─────────────────┐         │   X: [  400  ]        │
│  [Ch][Ea][Ac]  │        │                 │         │   Y: [  320  ]        │
│                │        │                 │         │                       │
│  ┌──┬──┬──┐    │        │   ◉  ◉         │         │   Scale               │
│  │  │  │  │    │        │     👃          │         │   W: [ 1.0 ] H:[1.0]  │
│  ├──┼──┼──┤    │        │    ───          │         │   🔗 Lock ratio       │
│  │  │  │  │    │        │                 │         │                       │
│  ├──┼──┼──┤    │        │                 │         │   Rotation            │
│  │  │  │  │    │        └─────────────────┘         │   [────●────] 0°      │
│  └──┴──┴──┘    │                                    │                       │
│                │                                    │   Opacity             │
│  ────────────  │                                    │   [━━━━━━━━━●] 100%   │
│  Drag feature  │                                    │                       │
│  to canvas     │                                    │   [↔ Flip H][↕ Flip V]│
│                │                                    │   [🔄 Reset Transform] │
│                │                                    │                       │
│                │                                    ├───────────────────────┤
│                │                                    │   Layers         [＋] │
│                │                                    │   ─────────────────   │
│                │                                    │   ☰ ◉ 🔓 Hair      🗑 │
│                │                                    │   ☰ ◉ 🔓 Eyes    ● 🗑 │
│                │                                    │   ☰ ◉ 🔓 Face      🗑 │
│                │                                    │   ☰ ◎ 🔒 Glasses   🗑 │
│                │                                    │   ─────────────────   │
│                │                                    │   [Group] [Ungroup]   │
├────────────────┴────────────────────────────────────┴───────────────────────┤
│  Case-2025-01*    800×1000    4 layers    Eyes selected     [−][100%][+][⛶]│
└─────────────────────────────────────────────────────────────────────────────┘
```

## With Compare Panel (Side)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  [←] [New][Open][Save] | [Export] [📷●]   Case-2025-01*     [↩][↪] [⋮]     │
├────────────┬────────────────────┬────────────────────┬──────────────────────┤
│            │                    │                    │                      │
│  Features  │    REFERENCE       │      CANVAS        │   Adjustments        │
│  ────────  │    ────────────    │    ────────────    │   ──────────────     │
│  [🔍...]   │  ┌──────────────┐  │  ┌──────────────┐  │   Selected: Eyes     │
│            │  │              │  │  │              │  │                      │
│  [Fa][Ha]  │  │              │  │  │   ◉  ◉      │  │   Position           │
│  [Ey][Br]  │  │  [Witness    │  │  │     👃       │  │   X:[400] Y:[320]    │
│  [No][Mo]  │  │   Photo]     │  │  │    ───       │  │                      │
│  [Ch][Ea]  │  │              │  │  │              │  │   Scale  Rotation    │
│  [Ac]      │  │              │  │  │              │  │   [1.0]  [0°]        │
│            │  └──────────────┘  │  └──────────────┘  │                      │
│  ┌──┬──┐   │                    │                    │   Opacity [━━━●]100% │
│  │  │  │   │  Opacity: [===●] 80%                   │                      │
│  ├──┼──┤   │  [🔗 Sync Zoom]    │                    │   [↔][↕][🔄 Reset]   │
│  │  │  │   │  [↔ Flip Ref]     │                    │                      │
│  └──┴──┘   │  [📷 Change Ref]   │                    ├──────────────────────┤
│            │  [✕ Close Compare] │                    │   Layers        [＋] │
│            │                    │                    │   ☰ ◉ Hair        🗑 │
│            │                    │                    │   ☰ ◉ Eyes      ● 🗑 │
│            │                    │                    │   ☰ ◉ Face        🗑 │
├────────────┴────────────────────┴────────────────────┴──────────────────────┤
│  Case-2025-01*    Compare: ON    4 layers    Eyes selected      [−][100%][+]│
└─────────────────────────────────────────────────────────────────────────────┘
```

## With Compare Overlay Mode

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  [←] [New][Open][Save] | [Export] [📷●]   Case-2025-01*     [↩][↪] [⋮]     │
├────────────────┬────────────────────────────────────┬───────────────────────┤
│                │                                    │                       │
│  Features      │   ┌─────────────────────────┐      │   Compare             │
│  ────────────  │   │                         │      │   ─────────────────   │
│  [🔍 Search..] │   │  ░░░░░░░░░░░░░░░░░░░░░  │      │   Mode: [Overlay ▾]   │
│                │   │  ░░░ REFERENCE ░░░░░░░  │      │   ┌─────────────────┐ │
│  [Fa][Ha][Ey]  │   │  ░░░ OVERLAID ░░░░░░░░  │      │   │ ○ Side by Side  │ │
│  [Br][No][Mo]  │   │  ░░░ ON CANVAS ░░░░░░░  │      │   │ ● Overlay       │ │
│  [Ch][Ea][Ac]  │   │  ░░░░░░░░░░░░░░░░░░░░░  │      │   │ ○ Onion Skin    │ │
│                │   │      ◉  ◉               │      │   │ ○ Split         │ │
│  ┌──┬──┬──┐    │   │        👃                │      │   └─────────────────┘ │
│  │  │  │  │    │   │       ───               │      │                       │
│  ├──┼──┼──┤    │   │                         │      │   Ref Opacity         │
│  │  │  │  │    │   │                         │      │   [━━━━●━━━━━] 50%    │
│  ├──┼──┼──┤    │   └─────────────────────────┘      │                       │
│  │  │  │  │    │                                    │   [🔗 Sync Position]  │
│  └──┴──┴──┘    │                                    │   [🔗 Sync Zoom]      │
│                │                                    │   [↔ Flip Reference]  │
│                │                                    │                       │
│                │                                    │   [📷 Change Ref]     │
│                │                                    │   [✕ Close Compare]   │
│                │                                    │                       │
│                │                                    ├───────────────────────┤
│                │                                    │   Layers         [＋] │
│                │                                    │   (layer list...)     │
├────────────────┴────────────────────────────────────┴───────────────────────┤
│  Case-2025-01*    Compare: Overlay 50%    4 layers              [−][100%][+]│
└─────────────────────────────────────────────────────────────────────────────┘
```

## Toolbar Icons Reference

| Icon | Meaning |
|------|---------|
| [←] | Back to Welcome |
| [📷] | Toggle Compare mode |
| [📷●] | Compare mode active |
| [↩] | Undo |
| [↪] | Redo |
| [⋮] | More menu |
| [⛶] | Fullscreen canvas |
| ◉ | Layer visible |
| ◎ | Layer hidden |
| 🔓 | Layer unlocked |
| 🔒 | Layer locked |
| ● | Selected layer indicator |

## Compare Modes

| Mode | Description |
|------|-------------|
| Side by Side | Reference and canvas next to each other |
| Overlay | Reference displayed on top of canvas |
| Onion Skin | Semi-transparent reference, like animation |
| Split | Diagonal split between reference and canvas |

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| C | Toggle Compare mode |
| 1-4 | Switch Compare mode |
| [ ] | Adjust reference opacity |
| Tab | Toggle side panels |

---

**Component**: `pages/EditorPage.tsx`
