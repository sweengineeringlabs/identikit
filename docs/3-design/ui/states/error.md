# UI States: Error States

## Application Crash

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                              ┌───────────────────────────────┐              │
│                              │                               │              │
│                              │              ⚠                │              │
│                              │                               │              │
│                              │   Something went wrong        │              │
│                              │                               │              │
│                              │   Identikit encountered an    │              │
│                              │   unexpected error and needs  │              │
│                              │   to restart.                 │              │
│                              │                               │              │
│                              │   Your work has been          │              │
│                              │   automatically saved.        │              │
│                              │                               │              │
│                              │   ─────────────────────────   │              │
│                              │                               │              │
│                              │   [📋 Copy Error Details]     │              │
│                              │                               │              │
│                              │   [🔄 Restart Application]    │              │
│                              │                               │              │
│                              │   [🐛 Report Issue]           │              │
│                              │                               │              │
│                              └───────────────────────────────┘              │
│                                                                             │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## File Open Error

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│        ┌─────────────────────────────────────────────────────────┐          │
│        │  Error Opening File                                     │          │
│        ├─────────────────────────────────────────────────────────┤          │
│        │                                                         │          │
│        │        ❌                                               │          │
│        │                                                         │          │
│        │   Unable to open file                                   │          │
│        │                                                         │          │
│        │   "Case-2025-001.idkit"                                 │          │
│        │                                                         │          │
│        │   ─────────────────────────────────────────────────     │          │
│        │                                                         │          │
│        │   Reason: File format not recognized                    │          │
│        │                                                         │          │
│        │   The file may be corrupted or from a newer             │          │
│        │   version of Identikit.                                 │          │
│        │                                                         │          │
│        │   Suggestions:                                          │          │
│        │   • Check if the file extension is .idkit               │          │
│        │   • Update Identikit to the latest version              │          │
│        │   • Try opening a backup copy                           │          │
│        │                                                         │          │
│        ├─────────────────────────────────────────────────────────┤          │
│        │              [View Details]              [OK]           │          │
│        └─────────────────────────────────────────────────────────┘          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## File Not Found

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│        ┌─────────────────────────────────────────────────────────┐          │
│        │  File Not Found                                         │          │
│        ├─────────────────────────────────────────────────────────┤          │
│        │                                                         │          │
│        │        📄❌                                             │          │
│        │                                                         │          │
│        │   The file could not be found                           │          │
│        │                                                         │          │
│        │   "Case-2025-001.idkit"                                 │          │
│        │   /home/user/Documents/composites/                      │          │
│        │                                                         │          │
│        │   The file may have been moved, renamed,                │          │
│        │   or deleted.                                           │          │
│        │                                                         │          │
│        ├─────────────────────────────────────────────────────────┤          │
│        │   [📂 Browse...]  [Remove from Recent]        [OK]      │          │
│        └─────────────────────────────────────────────────────────┘          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Save Error

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│        ┌─────────────────────────────────────────────────────────┐          │
│        │  Save Failed                                            │          │
│        ├─────────────────────────────────────────────────────────┤          │
│        │                                                         │          │
│        │        ⚠                                                │          │
│        │                                                         │          │
│        │   Unable to save file                                   │          │
│        │                                                         │          │
│        │   Error: Permission denied                              │          │
│        │                                                         │          │
│        │   Identikit cannot write to this location:              │          │
│        │   /protected/folder/Case-2025-001.idkit                 │          │
│        │                                                         │          │
│        │   Your work is preserved in memory.                     │          │
│        │                                                         │          │
│        ├─────────────────────────────────────────────────────────┤          │
│        │   [Save As...]  [Retry]                      [Cancel]   │          │
│        └─────────────────────────────────────────────────────────┘          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Export Error

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│        ┌─────────────────────────────────────────────────────────┐          │
│        │  Export Failed                                          │          │
│        ├─────────────────────────────────────────────────────────┤          │
│        │                                                         │          │
│        │        ❌                                               │          │
│        │                                                         │          │
│        │   Failed to export composite                            │          │
│        │                                                         │          │
│        │   Error: Insufficient disk space                        │          │
│        │                                                         │          │
│        │   Required: 24.5 MB                                     │          │
│        │   Available: 12.1 MB                                    │          │
│        │                                                         │          │
│        │   ─────────────────────────────────────────────────     │          │
│        │                                                         │          │
│        │   Suggestions:                                          │          │
│        │   • Free up disk space                                  │          │
│        │   • Choose a different location                         │          │
│        │   • Export at lower resolution                          │          │
│        │                                                         │          │
│        ├─────────────────────────────────────────────────────────┤          │
│        │        [Change Settings]  [Try Again]        [Cancel]   │          │
│        └─────────────────────────────────────────────────────────┘          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Feature Library Load Error

```
┌────────────────────────────┐
│ Feature Library        [×] │
├────────────────────────────┤
│                            │
│                            │
│          ⚠                 │
│                            │
│   Failed to load features  │
│                            │
│   Error: Invalid manifest  │
│   in /features/eyes/       │
│                            │
│   Some features may not    │
│   be available.            │
│                            │
│   [🔄 Retry]               │
│                            │
│   [View Details]           │
│                            │
└────────────────────────────┘
```

## Reference Image Load Error

```
┌────────────────────────────────────────┐
│ Compare Mode                       [×] │
├────────────────────────────────────────┤
│                                        │
│   Reference Image                      │
│   ─────────────────────────────────    │
│   ┌─────────────────────────────────┐  │
│   │                                 │  │
│   │              ❌                 │  │
│   │                                 │  │
│   │   Failed to load image          │  │
│   │                                 │  │
│   │   "reference.jpg" could not     │  │
│   │   be loaded. The file may be    │  │
│   │   corrupted or unsupported.     │  │
│   │                                 │  │
│   │   [Try Again]  [Choose Other]   │  │
│   │                                 │  │
│   └─────────────────────────────────┘  │
│                                        │
└────────────────────────────────────────┘
```

## Network Error (Update Check)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│        ┌─────────────────────────────────────────────────────────┐          │
│        │  Connection Error                                       │          │
│        ├─────────────────────────────────────────────────────────┤          │
│        │                                                         │          │
│        │        🌐❌                                             │          │
│        │                                                         │          │
│        │   Unable to check for updates                           │          │
│        │                                                         │          │
│        │   Could not connect to the update server.               │          │
│        │   Please check your internet connection.                │          │
│        │                                                         │          │
│        │   You can continue using Identikit offline.             │          │
│        │                                                         │          │
│        ├─────────────────────────────────────────────────────────┤          │
│        │                     [Retry]              [Skip]         │          │
│        └─────────────────────────────────────────────────────────┘          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Validation Error (Form)

```
┌────────────────────────────────────────┐
│ Case Information                       │
├────────────────────────────────────────┤
│                                        │
│   Case Number                          │
│   [                                 ]  │
│   ⚠ Case number is required            │
│                                        │
│   Author / Officer                     │
│   [                                 ]  │
│   ⚠ Please enter author name           │
│                                        │
│   Email                                │
│   [ invalid-email                   ]  │
│   ⚠ Please enter a valid email         │
│                                        │
│   ─────────────────────────────────    │
│                                        │
│   ⚠ Please fix 3 errors before saving  │
│                                        │
├────────────────────────────────────────┤
│                    [Cancel]    [Save]  │  ← Save disabled
└────────────────────────────────────────┘
```

## Canvas Render Error

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│     ┌───────────────────────────────────────────────────────────────────┐   │
│     │                                                                   │   │
│     │                                                                   │   │
│     │                              ⚠                                    │   │
│     │                                                                   │   │
│     │                   Canvas rendering failed                         │   │
│     │                                                                   │   │
│     │            WebGL context lost or unavailable                      │   │
│     │                                                                   │   │
│     │   This may be caused by:                                          │   │
│     │   • Graphics driver issues                                        │   │
│     │   • Low system resources                                          │   │
│     │   • Browser/system instability                                    │   │
│     │                                                                   │   │
│     │            [🔄 Reload Canvas]    [⚙ Troubleshoot]                 │   │
│     │                                                                   │   │
│     │                                                                   │   │
│     └───────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Feature Load Error (Single Item)

```
┌────────────────────────────┐
│ ▾ Eyes              (32)   │
│ ├────────────────────────┤ │
│ │                        │ │
│ │ ┌─────┐ ┌─────┐ ┌─────┐│ │
│ │ │👁️ 👁️ │ │ ⚠  │ │👁️ 👁️ ││ │  ← Failed to load
│ │ │     │ │Error│ │     ││ │
│ │ │Almon│ │     │ │Narro││ │
│ │ └─────┘ └─────┘ └─────┘│ │
│ │                        │ │
│ └────────────────────────┘ │
└────────────────────────────┘
```

## Inline Error Toast

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│                                                                             │
│                           [Main content area]                               │
│                                                                             │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│ ⚠ Failed to auto-save. Changes may be lost.            [Save Now] [Dismiss]│
└─────────────────────────────────────────────────────────────────────────────┘
```

## Error Details Modal

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│        ┌─────────────────────────────────────────────────────────┐          │
│        │  Error Details                                      [✕] │          │
│        ├─────────────────────────────────────────────────────────┤          │
│        │                                                         │          │
│        │   Error Code: ERR_FILE_CORRUPT                          │          │
│        │   Timestamp: 2025-12-27 14:32:15 UTC                    │          │
│        │                                                         │          │
│        │   Stack Trace:                                          │          │
│        │   ┌─────────────────────────────────────────────────┐   │          │
│        │   │ at parseIdkitFile (parser.rs:142)               │   │          │
│        │   │ at loadComposite (composite.rs:89)              │   │          │
│        │   │ at openFile (file_io.rs:56)                     │   │          │
│        │   │ at invoke_handler (main.rs:234)                 │   │          │
│        │   │                                                 │   │          │
│        │   │ Caused by: Invalid JSON at line 45, column 12   │   │          │
│        │   │ Expected '}', found ']'                         │   │          │
│        │   └─────────────────────────────────────────────────┘   │          │
│        │                                                         │          │
│        │   System Info:                                          │          │
│        │   OS: Windows 11 (10.0.22631)                           │          │
│        │   App Version: 1.0.0                                    │          │
│        │   Memory: 1.2 GB used                                   │          │
│        │                                                         │          │
│        ├─────────────────────────────────────────────────────────┤          │
│        │   [📋 Copy to Clipboard]    [🐛 Report Bug]    [Close]  │          │
│        └─────────────────────────────────────────────────────────┘          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

**Error Design Principles**:
- Clear, non-technical language for main message
- Technical details available but hidden by default
- Always provide a recovery action
- Include relevant context (file name, location)
- Never blame the user
- Offer to report bugs when appropriate
