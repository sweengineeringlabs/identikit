# UI States: Empty States

## Welcome Screen (No Recent Files)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ [≡] Identikit                                                          [⚙] │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
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
│                                                                             │
│                    ┌────────────────────────────────┐                       │
│                    │                                │                       │
│                    │       Welcome to Identikit     │                       │
│                    │                                │                       │
│                    │   Create accurate facial       │                       │
│                    │   composites from witness      │                       │
│                    │   descriptions                 │                       │
│                    │                                │                       │
│                    │   [+ New Composite]            │                       │
│                    │                                │                       │
│                    │   [📂 Open Existing...]        │                       │
│                    │                                │                       │
│                    └────────────────────────────────┘                       │
│                                                                             │
│                                                                             │
│                            No recent files                                  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Empty Canvas (New Composite)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ [≡] Identikit  │  File ▾  Edit ▾  View ▾  │  [↶][↷]  │  [C] Compare  │     │
├──────────────────┬──────────────────────────┬───────────────────────────────┤
│                  │                          │                               │
│  Feature Library │                          │   Adjustments                 │
│  ─────────────── │                          │   ─────────────────────────   │
│                  │     ┌────────────────┐   │                               │
│  ▸ Hair    (24)  │     │                │   │          ◇                    │
│  ▸ Face    (12)  │     │                │   │                               │
│  ▸ Eyes    (32)  │     │  Empty canvas  │   │   No layer selected           │
│  ▸ Eyebrows(18)  │     │                │   │                               │
│  ▸ Nose    (20)  │     │  Drag features │   │   Select a feature from       │
│  ▸ Mouth   (16)  │     │  from the left │   │   the library to begin        │
│  ▸ Chin     (8)  │     │  panel to      │   │   building your composite     │
│  ▸ Ears    (10)  │     │  get started   │   │                               │
│  ▸ Access. (28)  │     │                │   │                               │
│                  │     │     ⊕          │   │                               │
│                  │     │                │   ├───────────────────────────────┤
│                  │     └────────────────┘   │   Layers                      │
│                  │                          │   ─────────────────────────   │
│                  │                          │                               │
│                  │                          │        📑                     │
│                  │                          │                               │
│                  │                          │   No layers yet               │
│                  │                          │                               │
│                  │                          │   [Browse Features]           │
│                  │                          │                               │
└──────────────────┴──────────────────────────┴───────────────────────────────┘
```

## Empty Feature Library

```
┌────────────────────────────┐
│ Feature Library        [×] │
├────────────────────────────┤
│ [🔍 Search features...   ] │
├────────────────────────────┤
│                            │
│                            │
│                            │
│          📁                │
│                            │
│   No features found        │
│                            │
│   The feature library      │
│   is empty. Import         │
│   features to get started. │
│                            │
│   [📥 Import Features]     │
│                            │
│   [📖 Download Library]    │
│                            │
│                            │
│                            │
└────────────────────────────┘
```

## Empty Search Results

```
┌────────────────────────────┐
│ Feature Library        [×] │
├────────────────────────────┤
│ [🔍 rounded eyes       ✕ ] │
├────────────────────────────┤
│                            │
│                            │
│                            │
│          🔍                │
│                            │
│   No results found         │
│                            │
│   No features match        │
│   "rounded eyes"           │
│                            │
│   Suggestions:             │
│   • Try different keywords │
│   • Check spelling         │
│   • Use broader terms      │
│                            │
│   [Clear Search]           │
│                            │
│                            │
└────────────────────────────┘
```

## Empty Layers Panel

```
┌────────────────────────────┐
│ Layers                 [×] │
├────────────────────────────┤
│ [+ Add] [📁 Group] [🗑]    │
├────────────────────────────┤
│                            │
│                            │
│                            │
│          📑                │
│                            │
│   No layers yet            │
│                            │
│   Add features from the    │
│   library to create        │
│   your composite.          │
│                            │
│   Tip: Drag features       │
│   directly onto the        │
│   canvas, or click to      │
│   add them.                │
│                            │
│   [Browse Features]        │
│                            │
│                            │
└────────────────────────────┘
```

## Empty Adjustments Panel

```
┌────────────────────────────┐
│ Adjustments            [×] │
├────────────────────────────┤
│                            │
│                            │
│                            │
│          ◇                 │
│                            │
│   No layer selected        │
│                            │
│   Select a layer from the  │
│   canvas or layer panel    │
│   to adjust its properties │
│                            │
│   • Position & Size        │
│   • Rotation & Flip        │
│   • Opacity                │
│   • Color customization    │
│                            │
│                            │
│                            │
└────────────────────────────┘
```

## No Reference Image (Compare Mode)

```
┌────────────────────────────────────────┐
│ Compare Mode                       [×] │
├────────────────────────────────────────┤
│                                        │
│                                        │
│                                        │
│               📷                       │
│                                        │
│   No reference image                   │
│                                        │
│   Add a reference image to             │
│   compare your composite               │
│   against a witness photo              │
│   or other reference                   │
│                                        │
│   [📤 Upload Image]                    │
│                                        │
│   Or drag an image here                │
│                                        │
│   Supported: JPG, PNG, WEBP            │
│                                        │
│                                        │
└────────────────────────────────────────┘
```

## Empty Recent Files

```
┌────────────────────────────────────────┐
│ Recent Files                           │
├────────────────────────────────────────┤
│                                        │
│                                        │
│               📂                       │
│                                        │
│   No recent files                      │
│                                        │
│   Files you open or create             │
│   will appear here for                 │
│   quick access.                        │
│                                        │
│   [+ New Composite]                    │
│   [📂 Open File...]                    │
│                                        │
│                                        │
└────────────────────────────────────────┘
```

## Empty Custom Features

```
┌────────────────────────────┐
│ ▾ Custom Features      (0) │
│ ├────────────────────────┤ │
│ │                        │ │
│ │         📁             │ │
│ │                        │ │
│ │   No custom features   │ │
│ │                        │ │
│ │   Import your own      │ │
│ │   SVG features to      │ │
│ │   expand the library   │ │
│ │                        │ │
│ │  [📥 Import Feature]   │ │
│ │                        │ │
│ └────────────────────────┘ │
└────────────────────────────┘
```

## Empty History (Undo/Redo)

```
No actions to undo

[↶ Undo] ← Disabled, tooltip: "Nothing to undo (Ctrl+Z)"
[↷ Redo] ← Disabled, tooltip: "Nothing to redo (Ctrl+Y)"
```

## Empty Tags

```
┌────────────────────────────────────────┐
│ Tags                                   │
├────────────────────────────────────────┤
│                                        │
│   🏷                                   │
│                                        │
│   No tags added                        │
│                                        │
│   Tags help organize and               │
│   search your composites               │
│                                        │
│   [ + Add tag...                   ]   │
│                                        │
│   Suggestions:                         │
│   [male] [female] [caucasian]          │
│   [30-40] [glasses] [beard]            │
│                                        │
└────────────────────────────────────────┘
```

## Empty Export History

```
┌────────────────────────────────────────┐
│ Export History                         │
├────────────────────────────────────────┤
│                                        │
│               📤                       │
│                                        │
│   No exports yet                       │
│                                        │
│   Exported files will be               │
│   listed here for quick                │
│   reference                            │
│                                        │
│   [Export Current Composite]           │
│                                        │
└────────────────────────────────────────┘
```

## Empty Difference View (Identical Images)

```
┌─────────────────────────────────────────────────────────────────┐
│ Difference View                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                                                           │  │
│  │                                                           │  │
│  │                                                           │  │
│  │                      ✓                                    │  │
│  │                                                           │  │
│  │            No differences detected                        │  │
│  │                                                           │  │
│  │    The composite matches the reference image              │  │
│  │    with 0% difference                                     │  │
│  │                                                           │  │
│  │                                                           │  │
│  │                                                           │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                 │
│  Similarity: 100%                                               │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

**Design Principles**:
- All empty states include an icon
- Brief explanation of what belongs in the space
- Clear call-to-action when applicable
- Helpful tips where relevant
- Consistent styling across all panels
