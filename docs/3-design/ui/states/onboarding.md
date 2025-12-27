# UI States: Onboarding & Tutorials

## First Launch Welcome

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│                                                                             │
│                              ┌───────────────┐                              │
│                              │               │                              │
│                              │   IDENTIKIT   │                              │
│                              │               │                              │
│                              └───────────────┘                              │
│                                                                             │
│                           Facial Composite System                           │
│                                                                             │
│   ─────────────────────────────────────────────────────────────────────     │
│                                                                             │
│                         Welcome to Identikit!                               │
│                                                                             │
│      Create accurate facial composites from witness descriptions.           │
│      Let's take a quick tour to get you started.                            │
│                                                                             │
│                                                                             │
│                    ┌─────────────────────────────────┐                      │
│                    │                                 │                      │
│                    │       [🎓 Start Quick Tour]     │                      │
│                    │                                 │                      │
│                    │       [Skip - I know my way]    │                      │
│                    │                                 │                      │
│                    └─────────────────────────────────┘                      │
│                                                                             │
│                            ○ ○ ○ ○ ○  (Step 1 of 5)                         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Tour Step 1: Feature Library

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ [≡] Identikit  │  File ▾  Edit ▾  View ▾  │  [↶][↷]  │  [C] Compare  │     │
├──────────────────┬──────────────────────────┬───────────────────────────────┤
│ ╔════════════════╗                          │                               │
│ ║                ║                          │                               │
│ ║  Feature       ║    [Canvas dimmed]       │   [Panel dimmed]              │
│ ║  Library       ║                          │                               │
│ ║  ─────────     ║                          │                               │
│ ║                ║                          │                               │
│ ║  ▸ Hair (24)   ║                          │                               │
│ ║  ▸ Face (12)   ║                          │                               │
│ ║  ▸ Eyes (32)   ║                          │                               │
│ ║  ...           ║                          │                               │
│ ║                ║                          │                               │
│ ╠════════════════╣                          │                               │
│ ║                                                                           │
│ ║  📚 Feature Library                                                       │
│ ║                                                                           │
│ ║  Browse facial features organized by category.                            │
│ ║  Click a category to expand, then drag features                           │
│ ║  onto the canvas.                                                         │
│ ║                                                                           │
│ ║  [← Back]                    [Next: Canvas →]                             │
│ ║                                                                           │
│ ╚═══════════════════════════════════════════════════════════════════════════╝
│                            ● ○ ○ ○ ○  (Step 1 of 5)                         │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Tour Step 2: Canvas

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ [≡] Identikit  │  File ▾  Edit ▾  View ▾  │  [↶][↷]  │  [C] Compare  │     │
├──────────────────┬──────────────────────────┬───────────────────────────────┤
│                  │ ╔════════════════════════╗                               │
│  [Panel dimmed]  │ ║                        ║   [Panel dimmed]              │
│                  │ ║   Composite Canvas     ║                               │
│                  │ ║                        ║                               │
│                  │ ║   ┌────────────────┐   ║                               │
│                  │ ║   │                │   ║                               │
│                  │ ║   │   Drop zone    │   ║                               │
│                  │ ║   │   for features │   ║                               │
│                  │ ║   │                │   ║                               │
│                  │ ║   └────────────────┘   ║                               │
│                  │ ║                        ║                               │
│                  │ ╠════════════════════════╣                               │
│                  │ ║                                                        │
│                  │ ║  🎨 Canvas                                             │
│                  │ ║                                                        │
│                  │ ║  This is your workspace. Drag features here to         │
│                  │ ║  build your composite. Click to select, drag to        │
│                  │ ║  move, and use handles to resize and rotate.           │
│                  │ ║                                                        │
│                  │ ║  [← Back]                    [Next: Adjustments →]     │
│                  │ ╚════════════════════════════════════════════════════════╝
│                            ○ ● ○ ○ ○  (Step 2 of 5)                         │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Tour Step 3: Adjustment Panel

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ [≡] Identikit  │  File ▾  Edit ▾  View ▾  │  [↶][↷]  │  [C] Compare  │     │
├──────────────────┬──────────────────────────┬═══════════════════════════════┤
│                  │                          ║                               ║
│  [Panel dimmed]  │   [Canvas dimmed]        ║   Adjustments                 ║
│                  │                          ║   ─────────────               ║
│                  │                          ║                               ║
│                  │                          ║   Position                    ║
│                  │                          ║   X: [___]  Y: [___]          ║
│                  │                          ║                               ║
│                  │                          ║   Scale                       ║
│                  │                          ║   [━━━━━○━━━] 100%            ║
│                  │                          ║                               ║
│                  │                          ║   Rotation                    ║
│                  │                          ║   [___]°                      ║
│                  │                          ║                               ║
│                  │                          ╠═══════════════════════════════╣
│                  │                          ║                               ║
│                  │                          ║  ⚙ Adjustments                ║
│                  │                          ║                               ║
│                  │                          ║  Fine-tune each feature with  ║
│                  │                          ║  precise controls. Adjust     ║
│                  │                          ║  position, scale, rotation,   ║
│                  │                          ║  opacity, and colors.         ║
│                  │                          ║                               ║
│                  │                          ║  [← Back]    [Next: Layers →] ║
│                  │                          ╚═══════════════════════════════╝
│                            ○ ○ ● ○ ○  (Step 3 of 5)                         │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Tour Step 4: Layer Manager

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ [≡] Identikit  │  File ▾  Edit ▾  View ▾  │  [↶][↷]  │  [C] Compare  │     │
├──────────────────┬──────────────────────────┬───────────────────────────────┤
│                  │                          │                               │
│  [Panel dimmed]  │   [Canvas dimmed]        │   [Adjustments dimmed]        │
│                  │                          │                               │
│                  │                          ├═══════════════════════════════╣
│                  │                          ║   Layers                      ║
│                  │                          ║   ──────                      ║
│                  │                          ║                               ║
│                  │                          ║   ┌──────────────────────┐    ║
│                  │                          ║   │ [👁][🔓] Hair        │    ║
│                  │                          ║   ├──────────────────────┤    ║
│                  │                          ║   │ [👁][🔓] Eyes        │    ║
│                  │                          ║   ├──────────────────────┤    ║
│                  │                          ║   │ [👁][🔓] Face        │    ║
│                  │                          ║   └──────────────────────┘    ║
│                  │                          ║                               ║
│                  │                          ╠═══════════════════════════════╣
│                  │                          ║                               ║
│                  │                          ║  📑 Layer Manager             ║
│                  │                          ║                               ║
│                  │                          ║  Organize your features.      ║
│                  │                          ║  Drag to reorder, toggle      ║
│                  │                          ║  visibility, lock layers.     ║
│                  │                          ║                               ║
│                  │                          ║  [← Back]   [Next: Compare →] ║
│                  │                          ╚═══════════════════════════════╝
│                            ○ ○ ○ ● ○  (Step 4 of 5)                         │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Tour Step 5: Compare Mode

```
┌═════════════════════════════════════════════════════════════════════════════┐
║ [≡] Identikit  │  File ▾  Edit ▾  View ▾  │  [↶][↷]  │  [C] Compare ●│     ║
╠═════════════════════════════════════════════════════════════════════════════╣
║                                                                             ║
║     Compare your composite against a reference photo                        ║
║                                                                             ║
║     ┌─────────────────────────┬─────────────────────────┐                   ║
║     │                         │                         │                   ║
║     │      Reference          │       Composite         │                   ║
║     │                         │                         │                   ║
║     │        [Photo]          │        [Composite]      │                   ║
║     │                         │                         │                   ║
║     │                         │                         │                   ║
║     └─────────────────────────┴─────────────────────────┘                   ║
║                                                                             ║
║   View modes: Split | Overlay | Slider | Onion Skin | Difference           ║
║                                                                             ║
╠═════════════════════════════════════════════════════════════════════════════╣
║                                                                             ║
║  🔍 Compare Mode                                                            ║
║                                                                             ║
║  Upload a reference image (witness photo) and compare it                    ║
║  side-by-side with your composite. Use different view modes                 ║
║  to analyze the match. Press C to toggle compare mode.                      ║
║                                                                             ║
║  [← Back]                                          [✓ Finish Tour]          ║
║                                                                             ║
╚═════════════════════════════════════════════════════════════════════════════╝
                            ○ ○ ○ ○ ●  (Step 5 of 5)
```

## Tour Complete

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│                                                                             │
│                              ┌───────────────────────────────┐              │
│                              │                               │              │
│                              │             ✓                 │              │
│                              │                               │              │
│                              │   You're ready to go!         │              │
│                              │                               │              │
│                              │   You now know the basics.    │              │
│                              │   Start creating composites   │              │
│                              │   right away.                 │              │
│                              │                               │              │
│                              │   ─────────────────────────   │              │
│                              │                               │              │
│                              │   Quick Tips:                 │              │
│                              │   • Press F1 for shortcuts    │              │
│                              │   • Use Ctrl+Z to undo        │              │
│                              │   • Press C for compare mode  │              │
│                              │                               │              │
│                              │   [+ Create First Composite]  │              │
│                              │                               │              │
│                              │   [📖 View Full Documentation]│              │
│                              │                               │              │
│                              └───────────────────────────────┘              │
│                                                                             │
│                     [☐ Don't show this again]                               │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Contextual Tooltips

### Feature Hover Tip (First Time)
```
┌─────┐
│👁️ 👁️ │
│     │◄─────┬────────────────────────────────────────┐
│Almon│      │  💡 Tip: Drag features to canvas       │
└─────┘      │                                        │
             │  Click to preview, or drag directly    │
             │  onto the canvas to add this feature   │
             │  to your composite.                    │
             │                                        │
             │  [Got it]         [Don't show again]   │
             └────────────────────────────────────────┘
```

### Canvas First Drop Tip
```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│     ┌───────────────────────────────────────────────────────────────────┐   │
│     │                                                                   │   │
│     │                          👁️                                       │   │
│     │                    (newly added)                                  │   │
│     │                                                                   │   │
│     └───────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌───────────────────────────────────────────────────┐                      │
│  │  💡 Feature Added!                                │                      │
│  │                                                   │                      │
│  │  • Click to select                                │                      │
│  │  • Drag to move                                   │                      │
│  │  • Use corner handles to resize                   │                      │
│  │  • Use rotation handle to rotate                  │                      │
│  │                                                   │                      │
│  │  [Got it]                                         │                      │
│  └───────────────────────────────────────────────────┘                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Compare Mode First Use
```
┌────────────────────────────────────────────────────────────────┐
│  💡 Compare Mode                                               │
│                                                                │
│  Upload a reference image to compare with your composite.      │
│                                                                │
│  Different view modes help you analyze the match:              │
│  • Split: Side-by-side comparison                              │
│  • Overlay: Transparent overlay                                │
│  • Slider: Swipe between views                                 │
│  • Onion Skin: Ghost overlay with tints                        │
│  • Difference: Highlight differences                           │
│                                                                │
│  [Got it]                               [📤 Upload Reference]  │
└────────────────────────────────────────────────────────────────┘
```

## Keyboard Shortcuts Reminder

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│  ┌───────────────────────────────────────┐                                  │
│  │  ⌨ Keyboard Shortcuts                 │                                  │
│  │                                       │                                  │
│  │  Speed up your workflow:              │                                  │
│  │                                       │                                  │
│  │  Ctrl+S    Save                       │                                  │
│  │  Ctrl+Z    Undo                       │                                  │
│  │  Ctrl+D    Duplicate                  │                                  │
│  │  Delete    Remove layer               │                                  │
│  │  C         Toggle compare             │                                  │
│  │  F1        All shortcuts              │                                  │
│  │                                       │                                  │
│  │  [View All Shortcuts]                 │                                  │
│  └───────────────────────────────────────┘                                  │
│                                                                             │
│  Shows after user performs 5+ mouse-only operations                         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Settings: Onboarding Preferences

```
┌────────────────────────────────────────────────────────────────┐
│ Settings > Help & Tips                                         │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│   Onboarding                                                   │
│   ─────────────────────────────────────────────────────────    │
│                                                                │
│   [☑] Show welcome screen on first launch                      │
│   [☑] Show contextual tips for new features                    │
│   [☑] Show keyboard shortcut reminders                         │
│   [☐] Show tips for features I've used before                  │
│                                                                │
│   ─────────────────────────────────────────────────────────    │
│                                                                │
│   [🔄 Reset All Tips]                                          │
│   Show all tips again as if starting fresh                     │
│                                                                │
│   [🎓 Restart Tour]                                            │
│   Take the introductory tour again                             │
│                                                                │
│   ─────────────────────────────────────────────────────────    │
│                                                                │
│   Documentation                                                │
│   [📖 Open User Guide]                                         │
│   [📺 Video Tutorials]                                         │
│   [❓ FAQ]                                                     │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

**Onboarding Design Principles**:
- Non-intrusive - can be dismissed easily
- Progressive disclosure - don't overwhelm
- Contextual - appear when relevant
- Dismissible permanently with "Don't show again"
- Restartable from settings
- Highlight the UI element being explained
- Brief text, focus on actions not concepts
