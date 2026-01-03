#!/bin/bash
# Deploy script for reMarkable device

set -e

REMARKABLE_IP="${REMARKABLE_IP:-10.11.99.1}"
BINARY_NAME="jedusor"

echo "🔨 Building for reMarkable..."
cross build --release --target armv7-unknown-linux-gnueabihf

echo "📦 Binary size:"
ls -lh target/armv7-unknown-linux-gnueabihf/release/$BINARY_NAME

echo "🚀 Deploying to $REMARKABLE_IP..."
scp target/armv7-unknown-linux-gnueabihf/release/$BINARY_NAME root@$REMARKABLE_IP:/home/root/

echo "✅ Deployment complete!"
echo ""
echo "To run on device:"
echo "  ssh root@$REMARKABLE_IP"
echo "  ./$BINARY_NAME"
echo ""
echo "Or run directly:"
echo "  ssh root@$REMARKABLE_IP ./$BINARY_NAME"
