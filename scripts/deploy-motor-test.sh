#!/bin/sh
set -e

CAMERA_IP=${1:-"10.75.2.93"}
CAMERA_PORT=${2:-"2323"}
TARGET="arm-unknown-linux-musleabi"

echo "=== Building standalone motor-test for ARMv6 musl ==="
cargo build --release --target "$TARGET" --bin motor-test

BINARY="target/$TARGET/release/motor-test"

echo "=== Uploading motor-test to camera RAM ($CAMERA_IP) ==="
curl -u admin:admin -T "$BINARY" "http://$CAMERA_IP/tmpfs/motor-test"

echo "=== Upload complete! ==="
echo "Execute via Telnet root shell:"
echo "  telnet $CAMERA_IP $CAMERA_PORT"
echo "  chmod +x /tmpfs/motor-test"
echo "  /tmpfs/motor-test"
