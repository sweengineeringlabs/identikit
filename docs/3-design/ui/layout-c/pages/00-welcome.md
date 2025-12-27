# Screen: Welcome / No Composite (Layout C - Bottom Dock)

## Initial State

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  File  Edit  View  Help                                              [_][□][✕]          │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                         │
│                                                                                         │
│                                                                                         │
│                                                                                         │
│                                        👤                                               │
│                                                                                         │
│                                Welcome to Identikit                                     │
│                                                                                         │
│                         Professional Facial Composite Software                          │
│                                                                                         │
│                                                                                         │
│                     ┌─────────────────┐     ┌─────────────────┐                         │
│                     │  📄 New         │     │  📂 Open        │                         │
│                     │  Composite      │     │  File           │                         │
│                     └─────────────────┘     └─────────────────┘                         │
│                                                                                         │
│                                                                                         │
│                                    Recent Files                                         │
│                             ────────────────────────                                    │
│                             📄 suspect_01.idkit - 2 hours ago                           │
│                             📄 witness_desc.idkit - Yesterday                           │
│                             📄 case_2024_001.idkit - 3 days ago                         │
│                                                                                         │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│  Feature Library (Hidden until composite created)                                       │
│  ┌──────────────────────────────────────────────────────────────────────────────────┐   │
│  │                           No composite open                                       │   │
│  └──────────────────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## With Empty Recent Files

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  File  Edit  View  Help                                              [_][□][✕]          │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                         │
│                                                                                         │
│                                                                                         │
│                                                                                         │
│                                        👤                                               │
│                                                                                         │
│                                Welcome to Identikit                                     │
│                                                                                         │
│                         Professional Facial Composite Software                          │
│                                                                                         │
│                                                                                         │
│                     ┌─────────────────┐     ┌─────────────────┐                         │
│                     │  📄 New         │     │  📂 Open        │                         │
│                     │  Composite      │     │  File           │                         │
│                     └─────────────────┘     └─────────────────┘                         │
│                                                                                         │
│                                                                                         │
│                                    Recent Files                                         │
│                             ────────────────────────                                    │
│                                 No recent files                                         │
│                             Your composites will appear here                            │
│                                                                                         │
│                                                                                         │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│  [Feature Library - Disabled]                                                           │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Actions

| Element | Action |
|---------|--------|
| New Composite | Opens new composite modal |
| Open File | Opens file browser dialog |
| Recent file item | Opens that composite |
| Ctrl+N | New composite |
| Ctrl+O | Open file |

---

**Layout**: C (Bottom Dock)
**File**: `pages/Welcome.tsx`
**Route**: `/` (no composite loaded)
