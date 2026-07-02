#!/usr/bin/env python3
"""
PostToolUse hook — runs `rustfmt` then a compile check on a single edited
.rs file.

Format only, no clippy: this is a wasm-only (cdylib + wasm-bindgen) crate,
where host-target `cargo clippy` cannot link and would spam false failures.
`cargo check --target wasm32-unknown-unknown` does work here, and catches
real compile errors the rustfmt-only version of this hook could not — exits
2 on failure so Claude sees the diagnostics.
"""
import json
import os
import subprocess
import sys

data = json.load(sys.stdin)
file_path = (data.get("tool_input") or {}).get("file_path", "")

# Only process Rust source files
if not file_path.endswith(".rs"):
    sys.exit(0)

file_path = os.path.normpath(file_path)
if os.path.exists(file_path):
    # rustfmt edits the file in place; never fails the turn.
    subprocess.run(["rustfmt", "--edition", "2021", file_path])

# Find the repo root (nearest Cargo.toml walking up from the edited file).
search_dir = os.path.dirname(os.path.abspath(file_path))
root = None
while True:
    if os.path.exists(os.path.join(search_dir, "Cargo.toml")):
        root = search_dir
        break
    parent = os.path.dirname(search_dir)
    if parent == search_dir:  # reached filesystem root
        break
    search_dir = parent

if root is None:
    sys.exit(0)

r = subprocess.run(
    ["cargo", "check", "--target", "wasm32-unknown-unknown"],
    cwd=root,
    capture_output=True,
    text=True,
)
sys.stdout.write(r.stdout)
sys.stderr.write(r.stderr)
sys.stdout.flush()
sys.stderr.flush()

if r.returncode != 0:
    sys.exit(2)
