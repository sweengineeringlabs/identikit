# State: Error States (Layout D - Split View Focus)

## File Load Error

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  File  Edit  View  Help                                              [_][□][✕]          │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                         │
│                            ┌─────────────────────────────────┐                          │
│                            │              ⚠️                 │                          │
│                            │                                 │                          │
│                            │      Failed to Open File        │                          │
│                            │                                 │                          │
│                            │   The file could not be read.   │                          │
│                            │   It may be corrupted or in     │                          │
│                            │   an unsupported format.        │                          │
│                            │                                 │                          │
│                            │   File: suspect_01.idkit        │                          │
│                            │                                 │                          │
│                            │   [Show Details]   [OK]         │                          │
│                            │                                 │                          │
│                            └─────────────────────────────────┘                          │
│                                                                                         │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Feature Library Error

```
┌────────────────────────────────────────┬────────────────────────────────────────────────┐
│                                        │                                                │
│            ⚠️                          │                                                │
│                                        │                                                │
│  Feature Library Error                 │                                                │
│                                        │                                                │
│  Could not load feature library.       │                 CANVAS                         │
│  Some features may be unavailable.     │                                                │
│                                        │                                                │
│  [Retry]   [Continue Anyway]           │                                                │
│                                        │                                                │
│                                        │                                                │
│  ──────────────────────────────────────│                                                │
│  Layers                                │                                                │
│  [Available]                           │                                                │
│                                        │                                                │
└────────────────────────────────────────┴────────────────────────────────────────────────┘
```

## Export Error

```
┌────────────────────────────────────────┬────────────────────────────────────────────────┐
│                                        │                                                │
│                                        │            ┌───────────────────────┐           │
│                                        │            │           ❌          │           │
│  [Feature browser]                     │            │                       │           │
│                                        │            │     Export Failed     │           │
│                                        │            │                       │           │
│                                        │            │ Could not save file.  │           │
│                                        │            │ Permission denied.    │           │
│                                        │            │                       │           │
│                                        │            │ [Choose New Location] │           │
│                                        │            │      [Cancel]         │           │
│                                        │            │                       │           │
│                                        │            └───────────────────────┘           │
│  ──────────────────────────────────────│                                                │
│  Layers                                ├────────────────────────────────────────────────┤
│  [Available]                           │ Export failed                                  │
│                                        │                                                │
└────────────────────────────────────────┴────────────────────────────────────────────────┘
```

## Save Error

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│                                                                                         │
│                            ┌─────────────────────────────────┐                          │
│                            │              ⚠️                 │                          │
│                            │                                 │                          │
│                            │        Save Failed              │                          │
│                            │                                 │                          │
│                            │   Unable to save to:            │                          │
│                            │   /documents/composite.idkit    │                          │
│                            │                                 │                          │
│                            │   Disk may be full or           │                          │
│                            │   write-protected.              │                          │
│                            │                                 │                          │
│                            │   [Save As...] [Retry] [Cancel] │                          │
│                            │                                 │                          │
│                            └─────────────────────────────────┘                          │
│                                                                                         │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Unsaved Changes Warning

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│                                                                                         │
│                            ┌─────────────────────────────────┐                          │
│                            │              ⚠️                 │                          │
│                            │                                 │                          │
│                            │      Unsaved Changes            │                          │
│                            │                                 │                          │
│                            │   Do you want to save changes   │                          │
│                            │   to "suspect_01.idkit"?        │                          │
│                            │                                 │                          │
│                            │   Changes will be lost if you   │                          │
│                            │   don't save them.              │                          │
│                            │                                 │                          │
│                            │ [Don't Save] [Cancel] [Save]    │                          │
│                            │                                 │                          │
│                            └─────────────────────────────────┘                          │
│                                                                                         │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Reference Image Error

```
┌────────────────────────────────────────┐
│                                        │
│  Reference Image                       │
│  ──────────────────────                │
│                                        │
│            ⚠️                          │
│                                        │
│  Failed to load reference              │
│                                        │
│  The image format is not               │
│  supported or the file is              │
│  corrupted.                            │
│                                        │
│  Supported: JPG, PNG, BMP, TIFF        │
│                                        │
│  [Try Another Image]                   │
│                                        │
└────────────────────────────────────────┘
```

## Toast Error Notification

```
Position: Bottom-right

┌────────────────────────────────────────┬────────────────────────────────────────────────┐
│                                        │                                                │
│                                        │                                                │
│  [Feature browser]                     │              [CANVAS]                          │
│                                        │                                                │
│                                        │                          ┌──────────────────┐  │
│                                        │                          │ ⚠️ Auto-save     │  │
│                                        │                          │    failed    [✕] │  │
│  ──────────────────────────────────────│                          └──────────────────┘  │
│  Layers                                ├────────────────────────────────────────────────┤
│                                        │ [Adjustment bar]                               │
└────────────────────────────────────────┴────────────────────────────────────────────────┘
```

## Actions

| Error Type | Actions Available |
|------------|-------------------|
| File load | OK, Show Details |
| Feature library | Retry, Continue |
| Export | Choose new location, Cancel |
| Save | Save As, Retry, Cancel |
| Unsaved changes | Save, Don't Save, Cancel |
| Reference image | Try Another |
| Toast | Dismiss |

---

**Layout**: D (Split View Focus)
**Key Difference**: Errors in left panel or center modal overlays
