# Prerequisites

**Audience**: End Users, System Administrators

## WHAT: System Requirements

Minimum and recommended system requirements for running Identikit.

## System Requirements

### Minimum

| Component | Requirement |
|-----------|-------------|
| OS | Windows 10, macOS 11, Ubuntu 20.04 |
| RAM | 4 GB |
| Storage | 200 MB |
| Display | 1280x720 |

### Recommended

| Component | Requirement |
|-----------|-------------|
| OS | Windows 11, macOS 14, Ubuntu 22.04 |
| RAM | 8 GB |
| Storage | 500 MB |
| Display | 1920x1080 |

## Platform-Specific Requirements

### Windows

- Windows 10 version 1803 or later
- WebView2 Runtime (bundled with Windows 11, auto-installed on Windows 10)

### macOS

- macOS 11 (Big Sur) or later
- Apple Silicon or Intel processor

### Linux

- Modern Linux distribution with:
  - GTK 3
  - WebKitGTK 4.1
  - libayatana-appindicator3 (optional, for system tray)

#### Ubuntu/Debian

```bash
sudo apt install libwebkit2gtk-4.1-0 libgtk-3-0
```

#### Fedora

```bash
sudo dnf install webkit2gtk4.1 gtk3
```

## Related

- [Installation Guide](installation.md)

---

**Last Updated**: 2025-01-01
