#!/usr/bin/env bash
# Build and test every subsystem in this repository. Skips a subsystem
# (with a warning) instead of failing outright if its toolchain isn't
# installed, so this is safe to run on a partial dev setup.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FAILED=0

run_step() {
  local name="$1"; shift
  echo "== $name =="
  if ! ( "$@" ); then
    echo "!! $name FAILED"
    FAILED=1
  fi
  echo
}

if command -v cargo >/dev/null 2>&1; then
  run_step "kernel (Rust)"       bash -c "cd '$ROOT/kernel' && cargo build && cargo test"
  run_step "sync-engine (Rust)"  bash -c "cd '$ROOT/sync-engine' && cargo build && cargo test"
  run_step "sdk/rust"            bash -c "cd '$ROOT/sdk/rust/aether-sdk' && cargo test"
else
  echo "!! Skipping Rust components — cargo not found"
fi

if command -v cmake >/dev/null 2>&1 && command -v g++ >/dev/null 2>&1; then
  run_step "perception-engine (C++)" bash -c "cd '$ROOT/perception-engine' && cmake -B build && cmake --build build && ctest --test-dir build"
else
  echo "!! Skipping perception-engine — cmake/g++ not found"
fi

if command -v g++ >/dev/null 2>&1; then
  run_step "sdk/cpp" bash -c "cd '$ROOT/sdk/cpp' && g++ -std=c++20 -I include tests/sdk_tests.cpp -o /tmp/aether_cpp_sdk_tests && /tmp/aether_cpp_sdk_tests"
fi

if command -v python3 >/dev/null 2>&1; then
  run_step "intent-router (Python)" bash -c "cd '$ROOT/intent-router' && pip install -e '.[dev]' -q --break-system-packages 2>/dev/null; pip install -e '.[dev]' -q 2>/dev/null; python3 -m pytest -q"
  run_step "sdk/python"             bash -c "cd '$ROOT/sdk/python' && pip install -e '.[dev]' -q --break-system-packages 2>/dev/null; pip install -e '.[dev]' -q 2>/dev/null; python3 -m pytest -q"
else
  echo "!! Skipping Python components — python3 not found"
fi

if command -v npm >/dev/null 2>&1; then
  run_step "sdk/js" bash -c "cd '$ROOT/sdk/js/aether-sdk' && npm install --no-audit --no-fund && npm run build && npm test"
else
  echo "!! Skipping sdk/js — npm not found"
fi

if [ "$FAILED" -eq 0 ]; then
  echo "All available subsystems built and tested successfully."
else
  echo "One or more subsystems FAILED — see output above."
fi
exit "$FAILED"
