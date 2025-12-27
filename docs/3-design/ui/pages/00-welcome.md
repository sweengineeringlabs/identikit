# Page 00: Welcome Screen

## Standard View

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│                                                                             │
│                           ┌─────────────────┐                               │
│                           │   ◐◓◑           │                               │
│                           │   IDENTIKIT     │                               │
│                           │                 │                               │
│                           └─────────────────┘                               │
│                                                                             │
│                     Digital Facial Composite Platform                       │
│                                v0.1.0                                       │
│                                                                             │
│                                                                             │
│           ┌─────────────────────────────────────────────────────┐           │
│           │                                                     │           │
│           │   [ ＋ New Composite                             ]   │           │
│           │                                                     │           │
│           │   [ 📂 Open File...                              ]   │           │
│           │                                                     │           │
│           │   [ 📷 New from Reference Image...               ]   │           │
│           │                                                     │           │
│           └─────────────────────────────────────────────────────┘           │
│                                                                             │
│                                                                             │
│           Recent Composites                                    [View All]   │
│           ─────────────────────────────────────────────────────────────     │
│                                                                             │
│           ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌────────────┐ │
│           │  ┌────────┐  │ │  ┌────────┐  │ │  ┌────────┐  │ │ ┌────────┐ │ │
│           │  │        │  │ │  │        │  │ │  │        │  │ │ │        │ │ │
│           │  │ [Face] │  │ │  │ [Face] │  │ │  │ [Face] │  │ │ │ [Face] │ │ │
│           │  │        │  │ │  │        │  │ │  │        │  │ │ │        │ │ │
│           │  └────────┘  │ │  └────────┘  │ │  └────────┘  │ │ └────────┘ │ │
│           │              │ │              │ │              │ │            │ │
│           │ Case-2025-01 │ │ Suspect-A    │ │ Witness-3    │ │ John Doe   │ │
│           │ Dec 27, 2025 │ │ Dec 26, 2025 │ │ Dec 25, 2025 │ │ Dec 24     │ │
│           │ 📷 Has ref   │ │              │ │ 📷 Has ref   │ │            │ │
│           └──────────────┘ └──────────────┘ └──────────────┘ └────────────┘ │
│                                                                             │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  [⚙ Settings]    [❓ Help]    [📖 Tutorial]              [About Identikit] │
└─────────────────────────────────────────────────────────────────────────────┘
```

## First Launch (No Recent Files)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│                           ┌─────────────────┐                               │
│                           │   ◐◓◑           │                               │
│                           │   IDENTIKIT     │                               │
│                           │                 │                               │
│                           └─────────────────┘                               │
│                                                                             │
│                     Digital Facial Composite Platform                       │
│                                                                             │
│                                                                             │
│           ┌─────────────────────────────────────────────────────┐           │
│           │                                                     │           │
│           │   [ ＋ New Composite                             ]   │           │
│           │                                                     │           │
│           │   [ 📂 Open File...                              ]   │           │
│           │                                                     │           │
│           │   [ 📷 New from Reference Image...               ]   │           │
│           │                                                     │           │
│           └─────────────────────────────────────────────────────┘           │
│                                                                             │
│                                                                             │
│           ┌─────────────────────────────────────────────────────┐           │
│           │                                                     │           │
│           │              Welcome to Identikit!                  │           │
│           │                                                     │           │
│           │    Create facial composites with our comprehensive  │           │
│           │    feature library. Start by creating a new         │           │
│           │    composite or uploading a reference image.        │           │
│           │                                                     │           │
│           │              [ 🎓 Start Tutorial ]                  │           │
│           │                                                     │           │
│           └─────────────────────────────────────────────────────┘           │
│                                                                             │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  [⚙ Settings]    [❓ Help]    [📖 Tutorial]              [About Identikit] │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Specifications

| Element | Action |
|---------|--------|
| New Composite | Opens New Composite modal |
| Open File | Opens file picker (.idkit) |
| New from Reference | Opens reference upload, then editor |
| Recent File Card | Opens that composite in editor |
| View All | Opens full recent files list |
| Settings | Opens Settings page |
| Help | Opens Help page |
| Tutorial | Starts interactive onboarding |

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Ctrl+N | New Composite |
| Ctrl+O | Open File |
| Ctrl+R | New from Reference |
| F1 | Help |

---

**Component**: `pages/WelcomeScreen.tsx`
