#!/usr/bin/env python3
"""
PostToolUse hook — runs `rustfmt` on a single edited .rs file.

Format only, no clippy / no compile: safe in this wasm-only (cdylib +
wasm-bindgen) crate, where host-target `cargo clippy` cannot link and would
spam false failures.
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
