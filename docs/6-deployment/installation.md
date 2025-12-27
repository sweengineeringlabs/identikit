# Installation Guide

**Audience**: End Users, System Administrators

## WHAT: Installing Identikit

Step-by-step instructions for installing Identikit on different platforms.

## Download

Download the latest release from the [Releases](https://github.com/org/identikit/releases) page.

## Installation

### Windows

#### MSI Installer (Recommended)

1. Download `identikit-x.y.z-x64.msi`
2. Double-click to run the installer
3. Follow the installation wizard
4. Launch from Start Menu

#### Portable (No Install)

1. Download `identikit-x.y.z-x64.exe`
2. Run directly from any location

### macOS

#### DMG (Recommended)

1. Download `identikit-x.y.z-x64.dmg`
2. Open the DMG file
3. Drag Identikit to Applications folder
4. Launch from Applications

#### First Launch

On first launch, you may see a security warning. To allow:
1. Open System Preferences > Security & Privacy
2. Click "Open Anyway"

### Linux

#### AppImage (Universal)

```bash
# Download
wget https://github.com/org/identikit/releases/download/vX.Y.Z/identikit-x.y.z.AppImage

# Make executable
chmod +x identikit-x.y.z.AppImage

# Run
./identikit-x.y.z.AppImage
```

#### Debian/Ubuntu (.deb)

```bash
# Download
wget https://github.com/org/identikit/releases/download/vX.Y.Z/identikit_x.y.z_amd64.deb

# Install
sudo dpkg -i identikit_x.y.z_amd64.deb

# Fix dependencies if needed
sudo apt-get install -f
```

## Verify Installation

1. Launch Identikit
2. Create a new composite (File > New)
3. Add a feature from the library
4. Save the composite

## Uninstallation

### Windows

1. Open Settings > Apps
2. Find Identikit
3. Click Uninstall

### macOS

1. Drag Identikit from Applications to Trash
2. Empty Trash

### Linux (Debian)

```bash
sudo apt remove identikit
```

## Troubleshooting

### Windows: WebView2 Error

Install WebView2 Runtime from [Microsoft](https://developer.microsoft.com/en-us/microsoft-edge/webview2/).

### Linux: Missing Libraries

```bash
sudo apt install libwebkit2gtk-4.1-0 libgtk-3-0
```

### macOS: App is Damaged

```bash
xattr -cr /Applications/Identikit.app
```

## Related

- [Prerequisites](prerequisites.md)

---

**Last Updated**: 2025-01-01
