#!/usr/bin/env bash
# Aether Kernel — dev environment check/setup.
# Prints what's available and what's missing; does not require sudo
# unless installing packages, and never installs anything without
# telling you first.
set -euo pipefail

echo "== Aether Kernel dev environment check =="

check() {
  local name="$1" cmd="$2"
  if command -v "$cmd" >/dev/null 2>&1; then
    echo "  [ok]   $name ($("$cmd" --version 2>&1 | head -n1))"
  else
    echo "  [MISS] $name — install '$cmd' to build that component"
  fi
}

check "Rust/Cargo (kernel, sync-engine, sdk/rust)" cargo
check "CMake (perception-engine)"                  cmake
check "C++ compiler (perception-engine, sdk/cpp)"   g++
check "Python 3.10+ (intent-router, sdk/python)"    python3
check "Node.js 22+ (sdk/js)"                        node
check "npm (sdk/js)"                                npm

echo
echo "Per-module install commands:"
echo "  kernel/, sync-engine/, sdk/rust/aether-sdk : cargo build"
echo "  perception-engine/                         : cmake -B build && cmake --build build"
echo "  intent-router/, sdk/python                 : pip install -e '.[dev]'"
echo "  sdk/js/aether-sdk                          : npm install && npm run build"
echo
echo "Or just run: ./scripts/build_all.sh"
