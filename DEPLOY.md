# Jedusor Deployment Guide

## Automated Deployment with Launcher (Recommended)

The easiest way to deploy Jedusor with launcher integration from macOS:

```bash
# Set your device IP (default: 10.11.99.1 for USB)
export REMARKABLE_IP=10.11.99.1

# Build and deploy everything
./deploy-launcher.sh
```

This script will:
- Build the release binary for reMarkable
- Deploy binary and scripts to the device
- Install the .draft file for launcher integration
- Set proper permissions

After running this script, follow the on-screen instructions to install Toltec and Remux/Oxide if you haven't already.

## Quick Deploy (Manual)

```bash
# Build release binary
cargo zigbuild --release --target armv7-unknown-linux-gnueabihf

# Deploy to device (replace IP with your device's IP)
REMARKABLE_IP=192.168.1.40
scp target/armv7-unknown-linux-gnueabihf/release/jedusor root@$REMARKABLE_IP:/home/root/

# Run on device
ssh root@$REMARKABLE_IP ./start-jedusor.sh
```

## Running Modes

### Standard Mode (Recommended)
Uses `start-jedusor.sh` which automatically stops xochitl, runs Jedusor, then restarts xochitl:

```bash
ssh root@$REMARKABLE_IP ./start-jedusor.sh
```

**IMPORTANT**: Always run in exclusive mode (xochitl stopped). Running both Jedusor and xochitl simultaneously causes conflicts.

### Debug Mode
For troubleshooting with detailed logging:

```bash
ssh root@$REMARKABLE_IP ./run-debug.sh
```

Logs are saved to `/home/root/jedusor-debug.log`.

## Device Scripts

Three launcher scripts are available on the device:

1. **start-jedusor.sh** - Production mode (INFO logging)
2. **run-debug.sh** - Debug mode (DEBUG logging, saves to jedusor-debug.log)
3. **run-exclusive.sh** - Same as run-debug.sh (legacy name)

All scripts automatically handle stopping/starting xochitl.

## Launcher Integration (Optional)

For easier access from the reMarkable UI, install a launcher through Toltec:

### Install Toltec (Package Manager)

```bash
# SSH into device
ssh root@$REMARKABLE_IP

# Install Toltec
wget https://toltec-dev.org/bootstrap
bash bootstrap
```

### Install a Launcher

Choose one of these launchers:

**Option 1: Remux** (Recommended - simple and stable)
```bash
opkg install remux
systemctl enable --now remux
```

**Option 2: Oxide** (Full desktop environment)
```bash
opkg install oxide
systemctl enable --now tarnish
```

### Add Jedusor to Launcher

Once installed, launchers automatically discover apps in `/opt/bin/` or with `.draft` files. Your custom apps appear in the launcher menu - just tap to run.

**Note**: Launchers handle stopping xochitl automatically, so you don't need the start-jedusor.sh wrapper when launching from the UI.

## Controls

- **Draw**: Use stylus to write on screen
- **Circle gesture**: Draw a circle to trigger handwriting recognition
- **Exit**: Press middle button or power button

## Device IP Discovery

When connected via USB:
```bash
# reMarkable typically uses this IP over USB
ping 10.11.99.1

# Or over WiFi, find it in Settings > Help > Copyrights and licenses
# Look for IP address at bottom
```

## Build from macOS

```bash
# Install Zig-based cross-compiler (one time)
cargo install cargo-zigbuild

# Build for reMarkable
cargo zigbuild --release --target armv7-unknown-linux-gnueabihf
```

## Troubleshooting

### Strokes not appearing
- Make sure you're running in exclusive mode (xochitl stopped)
- Check logs: `ssh root@$REMARKABLE_IP cat /home/root/jedusor-debug.log`
- Look for "🎨 Drawing stroke with N points" - should show increasing point counts

### Can't connect to device
- USB: Try `ssh root@10.11.99.1` (default USB IP)
- WiFi: Check IP in device Settings
- Password: Found in Settings > Help > Copyrights and licenses

### App crashes immediately
- Check Rust version compatibility (MSRV 1.80+)
- Verify release build (debug builds have performance issues)
- Check logs for errors

### Recognition not working
- Needs network connection (Google Input Tools API)
- Draw clear strokes, then draw a circle gesture
- Check debug logs for API responses
