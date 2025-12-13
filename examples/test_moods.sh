#!/usr/bin/env bash
# Test script to demonstrate the mood system functionality

set -euo pipefail

echo "=== Testing mommy mood system ==="
echo ""

# Build the project first
echo "Building mommy..."
cargo build --release --quiet 2>&1 | grep -v "warning:" || true
echo ""

MOMMY="./target/release/mommy"

echo "Test 1: Default chill mood (positive response)"
SHELL_MOMMYS_NEEDY=1 "$MOMMY" 0
echo ""

echo "Test 2: Default chill mood (negative response)"
SHELL_MOMMYS_NEEDY=1 "$MOMMY" 1 || true
echo ""

echo "Test 3: Ominous mood (positive response)"
SHELL_MOMMYS_NEEDY=1 SHELL_MOMMYS_MOODS="ominous" "$MOMMY" 0
echo ""

echo "Test 4: Ominous mood (negative response)"
SHELL_MOMMYS_NEEDY=1 SHELL_MOMMYS_MOODS="ominous" "$MOMMY" 1 || true
echo ""

echo "Test 5: Thirsty mood (positive response)"
SHELL_MOMMYS_NEEDY=1 SHELL_MOMMYS_MOODS="thirsty" "$MOMMY" 0
echo ""

echo "Test 6: Thirsty mood (negative response)"
SHELL_MOMMYS_NEEDY=1 SHELL_MOMMYS_MOODS="thirsty" "$MOMMY" 1 || true
echo ""

echo "Test 7: Multiple moods rotation (5 samples)"
for i in {1..5}; do
    echo "  Sample $i:"
    SHELL_MOMMYS_NEEDY=1 SHELL_MOMMYS_MOODS="chill/ominous/thirsty" "$MOMMY" 0
done
echo ""

echo "Test 8: Using mommy with actual commands"
SHELL_MOMMYS_MOODS="chill" "$MOMMY" echo "Hello, world!"
echo ""

echo "Test 9: Fallback for unknown mood (should use default)"
SHELL_MOMMYS_NEEDY=1 SHELL_MOMMYS_MOODS="nonexistent" "$MOMMY" 0
echo ""

echo "=== All tests completed ==="
