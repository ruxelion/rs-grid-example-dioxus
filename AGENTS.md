# rs-grid-example-dioxus — Claude guide

Standalone demo of **rs-grid** with **Dioxus 0.7 (CSR)** + Trunk. App logic lives in a
single file: [`src/lib.rs`](src/lib.rs). Full build/run notes: [`README.md`](README.md).

## Quick reference

```sh
trunk serve            # dev → http://localhost:9081 (hot-reload)
trunk build --release  # → dist/
```

## Critical: this repo does NOT contain the library

`rs-grid-*` and `example-common` are **git dependencies pinned to a tag** (currently
`rs-grid-core-v0.1.3`, see [`Cargo.toml`](Cargo.toml)):

- The library source is in the separate `rs-grid` repo. Editing files here changes only
  the demo wiring in `src/lib.rs` — never grid behaviour.
- **All four deps must share the exact same tag.** Mixing per-crate tags breaks the build
  (`example-common` must match the library it was built against).
- To adopt a new library version: bump the tag on all four deps together, then `cargo update`.

## Conventions

- `themes/` is **vendored** from the rs-grid reference theme — re-vendor rather than hand-edit.
- Rust files are auto-formatted on save (PostToolUse `rustfmt` hook). No clippy hook: this
  is a `cdylib` + `wasm-bindgen` crate, so host-target clippy does not apply.
