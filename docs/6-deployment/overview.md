# Deployment Overview

**Audience**: DevOps, Release Managers

## WHAT: Deployment Options

Identikit can be deployed as a standalone desktop application for Windows, macOS, and Linux.

## Build Artifacts

| Platform | Format | Extension |
|----------|--------|-----------|
| Windows | MSI Installer | `.msi` |
| Windows | NSIS Installer | `.exe` |
| macOS | DMG | `.dmg` |
| macOS | App Bundle | `.app` |
| Linux | AppImage | `.AppImage` |
| Linux | Debian Package | `.deb` |

## Quick Build

```bash
cd frontend
bun run tauri build
```

Build artifacts are located in `backend/target/release/bundle/`.

## Documentation

| Document | Description |
|----------|-------------|
| [Prerequisites](prerequisites.md) | System requirements |
| [Installation](installation.md) | Installation guides |

## CI/CD

See `.github/workflows/` for automated build and release pipelines.

---

**Last Updated**: 2025-01-01
