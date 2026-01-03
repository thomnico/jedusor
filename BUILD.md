# Building Jedusor for reMarkable

## Prerequisites

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install cross** (recommended for easy cross-compilation):
   ```bash
   cargo install cross
   ```

   Alternative: Install the official reMarkable toolchain (more complex)

## Build Commands

### Option 1: Using `cross` (Recommended)

```bash
# Build release binary
cross build --release --target armv7-unknown-linux-gnueabihf

# Binary will be at:
# target/armv7-unknown-linux-gnueabihf/release/jedusor
```

### Option 2: Using standard `cargo` with ARM toolchain

```bash
# Install ARM target
rustup target add armv7-unknown-linux-gnueabihf

# Install ARM GCC (macOS with Homebrew)
brew install arm-none-eabi-gcc

# Build
cargo build --release --target armv7-unknown-linux-gnueabihf
```

## Deploy to reMarkable

### Manual Deployment

```bash
# Set your device IP (default when connected via USB)
export REMARKABLE_IP=10.11.99.1

# Copy binary to device
scp target/armv7-unknown-linux-gnueabihf/release/jedusor root@$REMARKABLE_IP:/home/root/

# SSH and run
ssh root@$REMARKABLE_IP
./jedusor
```

### Using Deploy Script

```bash
# Make deploy script executable (one-time)
chmod +x deploy.sh

# Deploy
./deploy.sh

# Or specify custom IP
REMARKABLE_IP=192.168.1.100 ./deploy.sh
```

## Running on Device

### Via SSH

```bash
ssh root@10.11.99.1 ./jedusor
```

### On Device Console

```bash
# SSH into device
ssh root@10.11.99.1

# Run app
./jedusor

# Exit app: Tap screen with finger
```

## Troubleshooting

### "cross: command not found"

Install cross:
```bash
cargo install cross
```

### "linker not found"

Install ARM toolchain:
```bash
# macOS
brew install arm-none-eabi-gcc

# Linux
sudo apt install gcc-arm-linux-gnueabihf
```

### "Permission denied" on device

Make binary executable:
```bash
ssh root@10.11.99.1 chmod +x jedusor
```

### App doesn't show anything

Check logs:
```bash
ssh root@10.11.99.1
RUST_LOG=info ./jedusor
```

## Current Features

✅ Real-time stylus stroke rendering
✅ Circle gesture detection
✅ Placeholder AI responses
✅ Touch-to-exit

## Next Steps

After confirming the app works:
1. Add handwriting recognition (Google Input Tools API)
2. Integrate Claude API for real responses
3. Add conversation history
4. Improve UI and animations
