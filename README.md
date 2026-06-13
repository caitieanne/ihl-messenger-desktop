# IHL Messenger — Desktop

A downloadable desktop app for **IHL Messenger**, built with [Tauri](https://tauri.app).
Think MSN / AOL / ICQ: a small native program that lives in your system tray.

The app wraps the live messenger (`https://industryhub.live/messenger-app`) in a
native window, so it always matches the web version — no separate frontend to
build or deploy. You sign in with your normal IHL account.

## Features

- Native window on **Windows, macOS, and Linux**, with its own icon
- **System tray** — closing the window hides it to the tray (classic IM behaviour); left-click or the tray menu reopens it
- **Single instance** — launching again focuses the existing window
- Tiny installer (~5–10 MB) — uses the OS webview, not a bundled browser

## Develop locally

Prerequisites: [Node.js](https://nodejs.org) 20+ and the
[Rust toolchain](https://www.rust-lang.org/tools/install), plus the
[Tauri system dependencies](https://tauri.app/start/prerequisites/) for your OS.

```bash
npm install
npm run dev      # run the app in development
npm run build    # build a production installer for your current OS
```

## Cutting a release (installers for all 3 platforms)

Installers are built automatically by GitHub Actions. To release:

```bash
# bump the version in src-tauri/tauri.conf.json and package.json, then:
git tag v0.1.0
git push origin v0.1.0
```

Pushing a `v*` tag runs `.github/workflows/release.yml`, which builds on
Windows, macOS (universal Apple Silicon + Intel), and Linux, and uploads the
installers to a **draft GitHub Release**:

- **Windows** — `.msi` and `.exe` (NSIS)
- **macOS** — `.dmg` (universal)
- **Linux** — `.AppImage` and `.deb`

Review the draft release, then publish it. You can also trigger the workflow
manually from the Actions tab (workflow_dispatch).

## Code signing (not yet configured)

The first releases are **unsigned**, so installers show a one-time
"unknown developer" warning. This is normal and safe to dismiss. To remove the
warnings later, add a Windows code-signing certificate and an Apple Developer
account + notarization (these are paid and account-specific). Until then,
unsigned installs work fine.

## Updating the icon

Replace `icon-source.png` (1024×1024) and regenerate:

```bash
npm run icon
```
