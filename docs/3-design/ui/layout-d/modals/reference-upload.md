# Modal: Reference Image Upload (Layout D - Split View Focus)

## Upload Dialog

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  Add Reference Image                                                        [✕]         │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                         │
│  ┌─────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                                                                 │   │
│  │                                    📷                                          │   │
│  │                                                                                 │   │
│  │                         Drag and drop an image here                            │   │
│  │                                                                                 │   │
│  │                                    or                                          │   │
│  │                                                                                 │   │
│  │                            [Browse Files...]                                   │   │
│  │                                                                                 │   │
│  │                    Supported formats: JPG, PNG, BMP, TIFF                      │   │
│  │                         Maximum size: 25 MB                                    │   │
│  │                                                                                 │   │
│  └─────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                         │
│                                                                [Cancel]                 │
│                                                                                         │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Preview and Confirm

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  Add Reference Image                                                        [✕]         │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                         │
│  ┌───────────────────────────────────────────┐  Image Details                          │
│  │                                           │  ─────────────────                      │
│  │                                           │                                          │
│  │                                           │  File: witness_photo.jpg                 │
│  │           [IMAGE PREVIEW]                 │  Size: 2.4 MB                            │
│  │                                           │  Dimensions: 1920 × 1280                 │
│  │                                           │                                          │
│  │                                           │  Alignment Options                       │
│  │                                           │                                          │
│  │                                           │  [● Auto fit to canvas]                  │
│  │                                           │  [○ Original size]                       │
│  │                                           │  [○ Custom scale: ____%]                 │
│  │                                           │                                          │
│  │                                           │  [☐] Center align with composite        │
│  └───────────────────────────────────────────┘                                          │
│                                                                                         │
│                                                       [Cancel]   [Use This Image]       │
│                                                                                         │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Actions

| Element | Action |
|---------|--------|
| Drop zone | Receive dropped images |
| [Browse Files...] | Open file picker |
| Alignment options | Set initial positioning |
| [Use This Image] | Confirm and add |
| [Cancel] | Close without changes |

---

**Layout**: D (Split View Focus)
**Trigger**: Compare mode > Upload reference
**Component**: `modals/ReferenceUploadModal.tsx`
