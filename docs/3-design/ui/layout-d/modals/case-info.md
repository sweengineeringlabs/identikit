# Modal: Case Information (Layout D - Split View Focus)

## Case Information Form

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  Case Information                                                           [✕]         │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                         │
│  [ Details ● ] [ Witness ] [ Notes ] [ Tags ]                                          │
│  ─────────────────────────────────────────────────────────────────────────────────────  │
│                                                                                         │
│  Case Details                                                                           │
│                                                                                         │
│  Case Number *                            Date                                          │
│  ┌──────────────────────────────────┐    ┌──────────────────────────────────┐          │
│  │ 2024-001                         │    │ 2024-01-15              📅       │          │
│  └──────────────────────────────────┘    └──────────────────────────────────┘          │
│                                                                                         │
│  Incident Type                                                                          │
│  ┌─────────────────────────────────────────────────────────────────────────────────┐   │
│  │ Armed Robbery                                                               ▾   │   │
│  └─────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                         │
│  ─────────────────────────────────────────────────────────────────────────────────────  │
│                                                                                         │
│  Personnel                                                                              │
│                                                                                         │
│  Officer/Artist *                         Badge Number                                  │
│  ┌──────────────────────────────────┐    ┌──────────────────────────────────┐          │
│  │ Det. James Smith                 │    │ 4521                             │          │
│  └──────────────────────────────────┘    └──────────────────────────────────┘          │
│                                                                                         │
│  Department                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────────────────┐   │
│  │ Metro Police Department - Criminal Investigations Division                       │   │
│  └─────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                         │
│                                                        [Cancel]   [Save Information]    │
│                                                                                         │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Notes Tab

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  Case Information                                                           [✕]         │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                         │
│  [ Details ] [ Witness ] [ Notes ● ] [ Tags ]                                          │
│  ─────────────────────────────────────────────────────────────────────────────────────  │
│                                                                                         │
│  Notes                                                                                  │
│  ┌─────────────────────────────────────────────────────────────────────────────────┐   │
│  │ Witness described suspect as male, approximately 35-40 years old.               │   │
│  │ Medium build, approximately 5'10".                                              │   │
│  │                                                                                  │   │
│  │ Notable features:                                                               │   │
│  │ - Scar above left eyebrow                                                       │   │
│  │ - Crooked nose (possibly previously broken)                                     │   │
│  │ - Thin lips                                                                     │   │
│  │                                                                                  │   │
│  │ Witness confidence: High (8/10) for facial features                             │   │
│  │                     Medium (5/10) for hair style                                │   │
│  │                                                                                  │   │
│  │                                                                                  │   │
│  │                                                                                  │   │
│  │                                                                                  │   │
│  │                                                                                  │   │
│  └─────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                         │
│  Characters: 423 / 2000                                                                 │
│                                                                                         │
│                                                        [Cancel]   [Save Information]    │
│                                                                                         │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Actions

| Element | Action |
|---------|--------|
| Tab navigation | Switch between sections |
| Text inputs | Enter case details |
| Date picker 📅 | Select dates |
| Dropdown fields | Select from options |
| Notes textarea | Free-form notes |
| [Save Information] | Save and close |
| [Cancel] | Discard changes |

---

**Layout**: D (Split View Focus)
**Trigger**: File > Case Information or Ctrl+I
**Component**: `modals/CaseInfoModal.tsx`
