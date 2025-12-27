# Screen: Welcome / No Composite (Layout D - Split View Focus)

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
│                                                                                         │
│                                                                                         │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## With Quick Start Options

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│  File  Edit  View  Help                                              [_][□][✕]          │
├─────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                         │
│                                        👤                                               │
│                                                                                         │
│                                Welcome to Identikit                                     │
│                                                                                         │
│                                                                                         │
│    ┌───────────────────────────────────────────────────────────────────────────────┐   │
│    │  Quick Start                                                                   │   │
│    │                                                                                │   │
│    │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │   │
│    │  │    📄        │  │    ♂         │  │    ♀         │  │    📂        │       │   │
│    │  │              │  │              │  │              │  │              │       │   │
│    │  │   Blank      │  │  Male Base   │  │ Female Base  │  │  Open File   │       │   │
│    │  │   Canvas     │  │  Template    │  │  Template    │  │              │       │   │
│    │  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘       │   │
│    │                                                                                │   │
│    └───────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                         │
│    ┌───────────────────────────────────────────────────────────────────────────────┐   │
│    │  Recent Files                                                                  │   │
│    │                                                                                │   │
│    │  📄 suspect_01.idkit ................ 2 hours ago                              │   │
│    │  📄 witness_desc.idkit .............. Yesterday                                │   │
│    │  📄 case_2024_001.idkit ............. 3 days ago                               │   │
│    │                                                                                │   │
│    └───────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                         │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

## Actions

| Element | Action |
|---------|--------|
| Blank Canvas | Create new empty composite |
| Male Base | Create with male template |
| Female Base | Create with female template |
| Open File | Browse for .idkit file |
| Recent file item | Open that composite |
| Ctrl+N | New composite |
| Ctrl+O | Open file |

---

**Layout**: D (Split View Focus)
**File**: `pages/Welcome.tsx`
**Route**: `/` (no composite loaded)
