#!/usr/bin/env bash
set -e
# Check if wasm-pack is installed
if ! [ -x "$(command -v wasm-pack)" ]; then
	echo "wasm-pack is not installed" >&2
	echo "Install it using:"
	echo "curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
	exit 1
fi

# Clean previous packages
if [ -d "pkg" ]; then
	rm -rf pkg
fi

wasm-pack build -t web -d pkg

