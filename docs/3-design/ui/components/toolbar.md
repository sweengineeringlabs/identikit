# Component: Main Toolbar

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
│                                                                                         │
│ Case-2025-001-Suspect ●                                              [💾 Save] [📤]    │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Menu Dropdowns

### File Menu
```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│ [≡] Identikit  │  File ▾  Edit ▾  View ▾  Help ▾  │  [↶][↷]  │  [C] Compare  │  [⚙]   │
│                   ┌──────────────────────┐                                              │
│                   │ New Composite   Ctrl+N│                                             │
│                   │ Open...        Ctrl+O│                                              │
│                   │ Open Recent       ▸  │─────────────────┐                            │
│                   ├──────────────────────┤ Case-2025-001   │                            │
│                   │ Save           Ctrl+S│ Case-2024-089   │                            │
│                   │ Save As...  Ctrl+Sh+S│ Case-2024-088   │                            │
│                   ├──────────────────────┤ ─────────────── │                            │
│                   │ Export         Ctrl+E│ Clear Recent    │                            │
│                   │ Print          Ctrl+P│─────────────────┘                            │
│                   ├──────────────────────┤                                              │
│                   │ Close          Ctrl+W│                                              │
│                   │ Exit           Alt+F4│                                              │
│                   └──────────────────────┘                                              │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

### Edit Menu
```
┌──────────────────────┐
│ Undo           Ctrl+Z│
│ Redo           Ctrl+Y│
├──────────────────────┤
│ Cut            Ctrl+X│
│ Copy           Ctrl+C│
│ Paste          Ctrl+V│
│ Duplicate      Ctrl+D│
│ Delete            Del│
├──────────────────────┤
│ Select All     Ctrl+A│
│ Deselect          Esc│
├──────────────────────┤
│ Preferences...  Ctrl,│
└──────────────────────┘
```

### View Menu
```
┌─────────────────────────┐
│ Zoom In           Ctrl++│
│ Zoom Out          Ctrl+-│
│ Zoom to Fit           F │
│ Actual Size       Ctrl+0│
├─────────────────────────┤
│ ☑ Feature Library       │
│ ☑ Adjustment Panel      │
│ ☑ Layer Manager         │
│ ☐ Reference Image       │
├─────────────────────────┤
│ Toggle Panels       Tab │
│ Fullscreen          F11 │
├─────────────────────────┤
│ Compare Mode          C │
│   Split               1 │
│   Overlay             2 │
│   Slider              3 │
│   Onion Skin          4 │
│   Difference          5 │
└─────────────────────────┘
```

### Help Menu
```
┌─────────────────────────┐
│ Getting Started         │
│ Documentation           │
│ Keyboard Shortcuts   F1 │
├─────────────────────────┤
│ Check for Updates       │
│ Report an Issue         │
├─────────────────────────┤
│ About Identikit         │
└─────────────────────────┘
```

## Undo/Redo States

### Both Available
```
[↶ Undo][↷ Redo]
```

### Only Undo Available
```
[↶ Undo][↷ Redo]  (Redo grayed out)
```

### Nothing to Undo
```
[↶ Undo][↷ Redo]  (Both grayed out)
```

### With Tooltip
```
                 ┌─────────────────────┐
[↶ Undo]         │ Undo: Move Eyes     │
                 │ Ctrl+Z              │
                 └─────────────────────┘
```

## Compare Toggle

### Compare Off
```
[C] Compare
```

### Compare Active
```
[C] Compare ●  Split ▾
              ┌───────────────┐
              │ ● Split       │
              │ ○ Overlay     │
              │ ○ Slider      │
              │ ○ Onion Skin  │
              │ ○ Difference  │
              └───────────────┘
```

## Responsive Variations

### Wide Screen (>1400px)
```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│ [≡] Identikit  │  File ▾  Edit ▾  View ▾  Help ▾  │  [↶ Undo][↷ Redo]  │  [C] Compare  │
│                                                                                         │
│ 📁 Case-2025-001-Suspect ●                                    [💾 Save] [📤 Export]    │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

### Medium Screen (1000-1400px)
```
┌─────────────────────────────────────────────────────────────────────────────┐
│ [≡]  │  File ▾  Edit ▾  View ▾  │  [↶][↷]  │  [C]  │  [💾][📤]  │  [⚙]   │
│ Case-2025-001-Suspect ●                                                     │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Narrow Screen (<1000px)
```
┌─────────────────────────────────────────────────────────┐
│ [≡]  │  [↶][↷]  │  [C]  │  [💾]  │  [⋮]              │
│ Case-2025-001-Suspect ●                                 │
└─────────────────────────────────────────────────────────┘

[⋮] Overflow Menu:
┌────────────────┐
│ Export         │
│ Settings       │
│ Help           │
└────────────────┘
```

---

**Component**: `components/toolbar/MainToolbar.tsx`
**Props**: `composite`, `canUndo`, `canRedo`, `compareMode`, `onMenuAction`
