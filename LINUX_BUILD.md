# Building on Linux

## Prerequisites

### 1. Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Tauri system dependencies (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libssl-dev \
  pkg-config \
  build-essential \
  curl \
  wget \
  libgtk-3-dev
```

For Arch:
```bash
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator gtk3 librsvg openssl
```

For Fedora:
```bash
sudo dnf install webkit2gtk4.1-devel libayatana-appindicator-gtk3-devel librsvg2-devel openssl-devel
```

### 3. Tauri CLI
```bash
cargo install tauri-cli
```

## Build

```bash
cd src-tauri
cargo tauri build
```

The output binary will be at:
`src-tauri/target/release/forzarichpresence`

Or as a `.deb`/`.AppImage` in:
`src-tauri/target/release/bundle/`

## Running in dev mode

```bash
cargo tauri dev
```

## Forza Setup (Linux / Steam)

Since Forza Horizon runs via Steam + Proton on Linux:

1. Open Forza Horizon 6 settings → HUD and Gameplay → Data Out
2. Set **Data Out IP Address** to `127.0.0.1`
3. Set **Data Out IP Port** to `8001` (or whatever port you set in the app)
4. Enable **Data Out**

No UWP loopback fix is needed — that's Windows-only. It just works.

## Discord

Make sure the Discord desktop app is running before launching this app.
Discord Rich Presence connects via a Unix socket automatically on Linux.
