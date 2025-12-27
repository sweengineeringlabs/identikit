# Modal: Export (Layout D - Split View Focus)

## Export Format Selection

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  Export Composite                                                           [✕]         │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                         │
│  Select Export Format                                                                   │
│  ─────────────────────────────────────────────────────────────────────────────────────  │
│                                                                                         │
│  ┌──────────────────────┐  ┌──────────────────────┐  ┌──────────────────────┐          │
│  │         📄           │  │         🖼️           │  │         📷           │          │
│  │                      │  │                      │  │                      │          │
│  │        PDF           │  │        PNG           │  │        JPEG          │          │
│  │    Full Document     │  │   Lossless Image     │  │  Compressed Image    │          │
│  │                      │  │                      │  │                      │          │
│  │  Best for printing   │  │  Best for quality    │  │  Best for sharing    │          │
│  │                      │  │                      │  │                      │          │
│  │       [ ● ]          │  │       [ ○ ]          │  │       [ ○ ]          │          │
│  └──────────────────────┘  └──────────────────────┘  └──────────────────────┘          │
│                                                                                         │
│                                                                     [Cancel]  [Next →]  │
│                                                                                         │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## PDF Options

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  Export as PDF                                                              [✕]         │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                         │
│  [← Back]                                                                               │
│                                                                                         │
│  ┌─────────────────────────────────────────┐  Document Settings                        │
│  │ ┌─────────────────────────────────────┐ │  ─────────────────────                    │
│  │ │  CASE #2024-001    [LOGO]          │ │                                            │
│  │ │                                     │ │  Paper Size                               │
│  │ │        [COMPOSITE PREVIEW]         │ │  [Letter (8.5" × 11")              ▾]     │
│  │ │                                     │ │                                            │
│  │ │  Officer: Det. Smith               │ │  Orientation                               │
│  │ │  Date: 2024-01-15                  │ │  [● Portrait]  [○ Landscape]              │
│  │ └─────────────────────────────────────┘ │                                            │
│  │              Page 1                     │  Quality                                   │
│  └─────────────────────────────────────────┘  [High (Print)                      ▾]     │
│                                                                                         │
│                                               Include                                   │
│                                               [☑] Case header with logo                 │
│                                               [☑] Case information                      │
│                                               [☐] Feature breakdown                     │
│                                               [☑] Date and timestamp                    │
│                                                                                         │
│  File: [suspect_01_composite                               ]  .pdf                      │
│  Location: [/Documents/Cases/2024                  ] [Browse...]                        │
│                                                                                         │
│  Estimated: ~2.1 MB                     [Cancel]  [← Back]  [Export PDF]               │
│                                                                                         │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## PNG Options

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  Export as PNG                                                              [✕]         │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                         │
│  [← Back]                                                                               │
│                                                                                         │
│  ┌─────────────────────────────────────────┐  Image Settings                           │
│  │                                         │  ─────────────────                        │
│  │          [COMPOSITE PREVIEW]            │                                            │
│  │                                         │  Resolution                               │
│  │              👁️    👁️                    │  [● 1× Original (800 × 1000)]             │
│  │                👃                        │  [○ 2× High DPI (1600 × 2000)]            │
│  │                👄                        │  [○ 4× Print (3200 × 4000)]               │
│  │                                         │  [○ Custom: ____ × ____]                  │
│  │                                         │                                            │
│  │                                         │  Background                                │
│  │                                         │  [● White (#FFFFFF)]                       │
│  │                                         │  [○ Transparent]                           │
│  │                                         │  [○ Custom color: ■ #______]              │
│  └─────────────────────────────────────────┘                                            │
│                                                                                         │
│  File: [suspect_01                                         ]  .png                      │
│  Location: [/Documents/Exports                     ] [Browse...]                        │
│                                                                                         │
│  Estimated: ~1.4 MB                     [Cancel]  [← Back]  [Export PNG]               │
│                                                                                         │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Actions

| Element | Action |
|---------|--------|
| Format cards | Select export type |
| [← Back] | Return to format selection |
| Paper/Resolution options | Set output dimensions |
| Quality dropdown | Set output quality |
| Include checkboxes | Toggle content sections |
| Background options | Set image background |
| [Browse...] | Choose save location |
| [Export X] | Begin export |

---

**Layout**: D (Split View Focus)
**Trigger**: File > Export or Ctrl+Shift+E
**Component**: `modals/ExportModal.tsx`
