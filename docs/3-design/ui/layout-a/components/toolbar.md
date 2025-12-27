# Component: Main Toolbar (Layout A - Horizontal Split)

## Default State

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│ [≡] Identikit  │  File ▾  Edit ▾  View ▾  Help ▾  │  [↶][↷]  │  [C] Compare  │  [⚙]   │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## With Active Composite

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│ [≡] Identikit  │  File ▾  Edit ▾  View ▾  Help ▾  │  [↶][↷]  │  [C] Compare  │  [⚙]   │
│ Case-2025-001-Suspect ●                                              [💾 Save] [📤]    │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Compare Mode Active

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│ [≡] Identikit  │  File ▾  Edit ▾  View ▾  Help ▾  │  [↶][↷]  │  [C] Compare ● │  [⚙]  │
│ [← Back to Editor]  Case-2025-001                                    [💾 Save] [📤]    │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## File Menu

```
┌───────────────────────────────────────────────────┐
│ File ▾                                            │
│ ┌──────────────────────┐                          │
│ │ New Composite   Ctrl+N│                         │
│ │ Open...        Ctrl+O│                          │
│ │ Open Recent       ▸  │                          │
│ ├──────────────────────┤                          │
│ │ Save           Ctrl+S│                          │
│ │ Save As...  Ctrl+Sh+S│                          │
│ ├──────────────────────┤                          │
│ │ Export         Ctrl+E│                          │
│ │ Print          Ctrl+P│                          │
│ ├──────────────────────┤                          │
│ │ Close          Ctrl+W│                          │
│ └──────────────────────┘                          │
└───────────────────────────────────────────────────┘
```

## View Menu (Layout A Specific)

```
┌─────────────────────────┐
│ Zoom In           Ctrl++│
│ Zoom Out          Ctrl+-│
│ Zoom to Fit           F │
├─────────────────────────┤
│ ☑ Feature Bar          │  ← Horizontal feature browser
│ ☑ Adjustment Panel      │
│ ☑ Layer Manager         │
│ ☐ Reference Image       │
├─────────────────────────┤
│ Collapse Feature Bar    │  ← Layout A specific
│ Toggle Panels       Tab │
│ Fullscreen          F11 │
├─────────────────────────┤
│ Compare Mode          C │
└─────────────────────────┘
```

---

**Layout**: A (Horizontal Split)
**Component**: `components/toolbar/MainToolbar.tsx`
**Key Difference**: View menu has "Feature Bar" instead of "Feature Library"
