# Modal: Confirmation Dialogs (Layout D - Split View Focus)

## Delete Layer Confirmation

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  Delete Layer                                                       [✕]     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│                              ⚠️                                             │
│                                                                             │
│              Are you sure you want to delete this layer?                    │
│                                                                             │
│                    "Eyes - Almond Style 1"                                  │
│                                                                             │
│              This action cannot be undone.                                  │
│                                                                             │
│                        [Cancel]    [Delete]                                 │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Unsaved Changes Warning

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  Unsaved Changes                                                    [✕]     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│                              ⚠️                                             │
│                                                                             │
│              Do you want to save changes to                                 │
│              "suspect_01.idkit" before closing?                             │
│                                                                             │
│              Your changes will be lost if you don't save them.              │
│                                                                             │
│               [Don't Save]    [Cancel]    [Save]                            │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Replace Feature Confirmation

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  Replace Feature                                                    [✕]     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│              A feature of this type already exists.                         │
│                                                                             │
│              Do you want to replace "Eyes - Round Style"                    │
│              with "Eyes - Almond Style 1"?                                  │
│                                                                             │
│              [☐] Keep existing position and scale                           │
│                                                                             │
│                [Cancel]    [Add Anyway]    [Replace]                        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Reset All Confirmation

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  Reset All Properties                                               [✕]     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│              Reset all properties to default values?                        │
│                                                                             │
│              This will reset: Position, Scale, Rotation,                    │
│              Opacity, and Colors                                            │
│                                                                             │
│                        [Cancel]    [Reset All]                              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Actions

| Dialog | Primary Action | Secondary Action |
|--------|----------------|------------------|
| Delete Layer | [Delete] | [Cancel] |
| Unsaved Changes | [Save] | [Don't Save], [Cancel] |
| Replace Feature | [Replace] | [Add Anyway], [Cancel] |
| Reset All | [Reset All] | [Cancel] |

---

**Layout**: D (Split View Focus)
**Component**: `modals/ConfirmDialog.tsx`
