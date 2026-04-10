#!/usr/bin/env bash
set -euo pipefail

usage() {
    echo "Usage: $0 <js-py-version> <rust-version>"
    echo "  e.g. $0 1.0.21 0.1.21"
    exit 1
}

[[ $# -ne 2 ]] && usage

JS_VER="$1"
RUST_VER="$2"

# Validate version format (digits and dots only)
[[ "$JS_VER" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || { echo "Invalid JS/Python version: $JS_VER"; exit 1; }
[[ "$RUST_VER" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || { echo "Invalid Rust version: $RUST_VER"; exit 1; }

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Detect current versions from canonical sources
OLD_RUST_VER="$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$ROOT/Cargo.toml" | head -1)"
OLD_JS_VER="$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$ROOT/bindings/nodejs/Cargo.toml" | head -1)"

echo "Bumping Rust crates:    $OLD_RUST_VER -> $RUST_VER"
echo "Bumping JS/Py packages: $OLD_JS_VER -> $JS_VER"

# Files with Rust crate version (0.1.x)
RUST_FILES=(
    "$ROOT/Cargo.toml"
    "$ROOT/bindings/python/Cargo.toml"
    "$ROOT/bindings/wasm/Cargo.toml"
    "$ROOT/bindings/dotnet/Cargo.toml"
)

# Files with JS/Python package version (1.0.x)
JS_FILES=(
    "$ROOT/bindings/nodejs/Cargo.toml"
    "$ROOT/bindings/nodejs/package.json"
    "$ROOT/bindings/nodejs/package-lock.json"
    "$ROOT/bindings/nodejs/pkgs/sentencex-darwin-arm64/package.json"
    "$ROOT/bindings/nodejs/pkgs/sentencex-darwin-x64/package.json"
    "$ROOT/bindings/nodejs/pkgs/sentencex-linux-arm64/package.json"
    "$ROOT/bindings/nodejs/pkgs/sentencex-linux-x64/package.json"
    "$ROOT/bindings/nodejs/pkgs/sentencex-win32-x64/package.json"
    "$ROOT/bindings/python/pyproject.toml"
)

for f in "${RUST_FILES[@]}"; do
    sed -i "s/^version = \"$OLD_RUST_VER\"/version = \"$RUST_VER\"/" "$f"
    echo "  updated $f"
done

for f in "${JS_FILES[@]}"; do
    sed -i "s/\"$OLD_JS_VER\"/\"$JS_VER\"/g" "$f"
    echo "  updated $f"
done

# Regenerate Cargo.lock
echo "Running cargo update --workspace..."
cargo update --workspace --quiet 2>&1 | grep -v "^$" || true

echo "Done. Verify changes with: git diff"
echo "Remember to update CHANGELOG.md before committing."
