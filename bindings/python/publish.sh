#!/bin/bash
# Build and publish at pypi
uv tool run maturin build -i python3.12
uv tool run maturin build -i python3.13
uv tool run maturin build -i python3.14
uv tool run maturin publish
