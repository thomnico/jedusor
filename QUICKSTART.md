# Jedusor Quick Start Guide

## What You Have

A minimal working Journal Mode app for reMarkable that:
- ✅ Captures stylus strokes in real-time
- ✅ Detects circle gestures
- ✅ Shows placeholder AI responses
- ✅ Touch screen to exit

## Build & Deploy (3 Steps)

### 1. Install Cross-Compilation Tool

```bash
cargo install cross
```

This will take 2-5 minutes. `cross` makes building for reMarkable much easier than manual toolchain setup.

### 2. Build Release Binary

```bash
cross build --release --target armv7-unknown-linux-gnueabihf
```

First build takes 5-10 minutes (downloads dependencies). Subsequent builds are faster.

The binary will be at: `target/armv7-unknown-linux-gnueabihf/release/jedusor`

### 3. Deploy to reMarkable

```bash
# Connect reMarkable via USB
# Make sure USB web interface is enabled in Settings > Storage

export REMARKABLE_IP=10.11.99.1

# Copy binary
scp target/armv7-unknown-linux-gnueabihf/release/jedusor root@$REMARKABLE_IP:/home/root/

# Run it!
ssh root@$REMARKABLE_IP ./jedusor
```

## Using the App

1. **Draw with stylus** - Your strokes appear in real-time
2. **Draw a circle** - AI placeholder response appears
3. **Tap screen** - Exit the app

## What to Expect

When you run `./jedusor`, you should see:
- Welcome message at top
- "Draw a circle to trigger AI response"
- "Double-tap to exit"
- Your stylus strokes appearing as you draw
- Text response when you complete a circle gesture

## Troubleshooting

**"cross: command not found"**
```bash
cargo install cross
# Wait for installation to complete
```

**"Connection refused" when copying to device**
```bash
# Check USB connection
# Enable USB web interface: Settings > Storage > USB web interface
ping 10.11.99.1
```

**"Permission denied" when running**
```bash
ssh root@10.11.99.1 chmod +x jedusor
ssh root@10.11.99.1 ./jedusor
```

**App doesn't display anything**
```bash
# Check logs
ssh root@10.11.99.1
RUST_LOG=debug ./jedusor
```

## Next Steps

Once you confirm the basic app works:
1. Add real handwriting recognition
2. Integrate Claude API
3. Build conversation context
4. Add more gestures and UI improvements

See BUILD.md for detailed build options and README.md for full documentation.
