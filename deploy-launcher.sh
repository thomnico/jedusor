#!/bin/bash
# Deploy Jedusor with launcher integration
# Run from macOS to deploy to reMarkable device

set -e

# Configuration
REMARKABLE_IP="${REMARKABLE_IP:-10.11.99.1}"
BUILD_TARGET="armv7-unknown-linux-gnueabihf"
DRAFT_DIR="/opt/etc/draft"

echo "🚀 Deploying Jedusor with launcher integration to $REMARKABLE_IP"

# Step 1: Build release binary
echo "📦 Building release binary..."
cargo zigbuild --release --target $BUILD_TARGET

# Step 2: Deploy binary
echo "📤 Deploying binary to /home/root/jedusor..."
scp target/$BUILD_TARGET/release/jedusor root@$REMARKABLE_IP:/home/root/

# Step 3: Deploy startup script
echo "📤 Deploying startup script..."
scp start-jedusor.sh root@$REMARKABLE_IP:/home/root/
ssh root@$REMARKABLE_IP "chmod +x /home/root/start-jedusor.sh"

# Step 4: Deploy .draft file
echo "📤 Deploying launcher integration (.draft file)..."
ssh root@$REMARKABLE_IP "mkdir -p $DRAFT_DIR"
scp jedusor.draft root@$REMARKABLE_IP:$DRAFT_DIR/

# Step 5: Set permissions
echo "🔐 Setting permissions..."
ssh root@$REMARKABLE_IP "chmod +x /home/root/jedusor"

echo ""
echo "✅ Deployment complete!"
echo ""
echo "📝 Next steps:"
echo "   1. Install Toltec package manager (if not already installed):"
echo "      ssh root@$REMARKABLE_IP"
echo "      wget https://toltec-dev.org/bootstrap"
echo "      bash bootstrap"
echo ""
echo "   2. Install Remux launcher (recommended):"
echo "      opkg install remux"
echo "      systemctl enable --now remux"
echo ""
echo "   3. Jedusor should now appear in your launcher menu!"
echo ""
echo "   Alternative: Install Oxide (full desktop):"
echo "      opkg install oxide"
echo "      systemctl enable --now tarnish"
echo ""
