# UI States: Onboarding (Layout A - Horizontal Split)

## Tour Step 1: Feature Bar

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ [≡] Identikit  │ File ▾ Edit ▾ View ▾ │ [↶][↷] │ [C] Compare │             │
├═════════════════════════════════════════════════════════════════════════════┤
║ Features: [Hair ▾][Face ▾][Eyes ▾][Brows ▾][Nose ▾][Mouth ▾][Ears ▾][+ ▾]  ║
║ ┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐ →         ║
║ │     ││     ││     ││     ││     ││     ││     ││     ││     │           ║
╠═════════════════════════════════════════════════════════════════════════════╣
║                                                                             ║
║  📚 Feature Bar                                                             ║
║                                                                             ║
║  In this layout, features are displayed in a horizontal bar at the top.    ║
║  Click a category dropdown to browse, or search across all categories.     ║
║  Drag features directly onto the canvas below.                             ║
║                                                                             ║
║  [← Back]                                          [Next: Canvas →]         ║
║                                                                             ║
╚═════════════════════════════════════════════════════════════════════════════╝
                            ● ○ ○ ○ ○  (Step 1 of 5)
```

## Tour Step 2: Canvas

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ [≡] Identikit  │ File ▾ Edit ▾ View ▾ │ [↶][↷] │ [C] Compare │             │
├─────────────────────────────────────────────────────────────────────────────┤
│ Features: [Dimmed...]                                                       │
├═════════════════════════════════════════════════════════════════════════════┤
║                                              │                              ║
║                                              │  [Dimmed]                    ║
║     ╔════════════════════════════════╗       │                              ║
║     ║                                ║       │                              ║
║     ║         CANVAS AREA            ║       │                              ║
║     ║                                ║       │                              ║
║     ║    Drop features here          ║       │                              ║
║     ║                                ║       │                              ║
║     ╚════════════════════════════════╝       │                              ║
║                                              │                              ║
╠══════════════════════════════════════════════╧══════════════════════════════╣
║                                                                             ║
║  🎨 Canvas                                                                  ║
║                                                                             ║
║  The canvas takes up more vertical space in this layout.                    ║
║  Drag features from the bar above, click to select, use handles to          ║
║  resize and rotate.                                                         ║
║                                                                             ║
║  [← Back]                                      [Next: Adjustments →]        ║
║                                                                             ║
╚═════════════════════════════════════════════════════════════════════════════╝
                            ○ ● ○ ○ ○  (Step 2 of 5)
```

## Tour Step 3: Tabbed Panels

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ [≡] Identikit  │ File ▾ Edit ▾ View ▾ │ [↶][↷] │ [C] Compare │             │
├─────────────────────────────────────────────────────────────────────────────┤
│ Features: [Dimmed...]                                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                              │╔════════════════════════════╗│
│                                              │║ [Adjustments] [Layers]     ║│
│     [Dimmed]                                 │║                            ║│
│                                              │║ Position, Scale, Rotation  ║│
│                                              │║ Layer management           ║│
│                                              │║                            ║│
│                                              │╠════════════════════════════╣│
│                                              │║                            ║│
│                                              │║  ⚙ Tabbed Panels           ║│
│                                              │║                            ║│
│                                              │║  Adjustments and Layers    ║│
│                                              │║  share the same panel.     ║│
│                                              │║  Click tabs to switch.     ║│
│                                              │║                            ║│
│                                              │║  [← Back]  [Next →]        ║│
│                                              │╚════════════════════════════╝│
└──────────────────────────────────────────────┴──────────────────────────────┘
                            ○ ○ ● ○ ○  (Step 3 of 5)
```

## Tour Complete

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│                              ┌───────────────────────────────┐              │
│                              │             ✓                 │              │
│                              │                               │              │
│                              │   You're ready to go!         │              │
│                              │                               │              │
│                              │   Layout A Tips:              │              │
│                              │   • Use dropdowns to browse   │              │
│                              │   • Collapse bar for more     │              │
│                              │     canvas space              │              │
│                              │   • Switch tabs on the right  │              │
│                              │                               │              │
│                              │   [+ Create First Composite]  │              │
│                              │                               │              │
│                              └───────────────────────────────┘              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Feature Bar Tooltip

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│ Features: [Hair ▾][Face ▾][Eyes ▼][Brows ▾]...                                         │
│                        │                                                                │
│                        ▼                                                                │
│           ┌───────────────────────────────────────────────────┐                         │
│           │  💡 Tip: Click category to browse features        │                         │
│           │                                                   │                         │
│           │  Each dropdown shows all features in that         │                         │
│           │  category. Drag any feature to the canvas.        │                         │
│           │                                                   │                         │
│           │  [Got it]                     [Don't show again]  │                         │
│           └───────────────────────────────────────────────────┘                         │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

---

**Layout**: A (Horizontal Split)
**States**: Onboarding states adapted for horizontal layout
