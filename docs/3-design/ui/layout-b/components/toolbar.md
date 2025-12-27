# Component: Main Toolbar (Layout B - Floating Panels)

## Default State

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│ [≡] Identikit │ File ▾ Edit ▾ View ▾ Help ▾ │ [↶][↷] │ [C] │ [📚][⚙][📑] │ [⚙]        │
│                                                         ↑    ↑    ↑                     │
│                                                     Features│   │Layers                │
│                                                         Adjustments                     │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## With Active Composite

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│ [≡] Identikit │ File ▾ Edit ▾ View ▾ │ [↶][↷] │ [C] │ [📚][⚙][📑] │ [⚙]               │
│ Case-2025-001-Suspect ●                                              [💾 Save] [📤]    │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Panel Toggle States

### All Panels Visible
```
│ [📚●][⚙●][📑●] │  ← Blue dot indicates panel is open
```

### Some Panels Hidden
```
│ [📚][⚙●][📑] │  ← Only Adjustments panel visible
```

### All Panels Minimized
```
│ [📚][⚙][📑] │  ← No panels visible (maximum canvas space)
```

## View Menu (Layout B Specific)

```
┌─────────────────────────┐
│ Zoom In           Ctrl++│
│ Zoom Out          Ctrl+-│
│ Zoom to Fit           F │
├─────────────────────────┤
│ Panels                ▸ │─────────────────────┐
│                         │ ☑ Features      F5  │
│ Reset Panel Positions   │ ☑ Adjustments   F6  │
│ Save Panel Layout       │ ☑ Layers        F7  │
├─────────────────────────│ ───────────────────│
│ Grid                [☑] │ Reset Positions     │
│ Snap to Grid        [☑] │ Save Layout         │
├─────────────────────────│ Minimize All    Esc │
│ Fullscreen          F11 │─────────────────────┘
├─────────────────────────┤
│ Compare Mode          C │
└─────────────────────────┘
```

## Panel Toggle Tooltips

```
     ┌───────────────────────────┐
[📚] │ Feature Library (F5)     │
     │ Click to show/hide panel │
     └───────────────────────────┘
```

---

**Layout**: B (Floating Panels)
**Component**: `components/toolbar/MainToolbar.tsx`
**Key Difference**: Panel toggle icons in toolbar for quick show/hide
