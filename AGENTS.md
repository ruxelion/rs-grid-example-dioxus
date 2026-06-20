# rs-grid-example-dioxus — Claude guide

Full-featured demo of **rs-grid** with **Dioxus 0.7 (CSR)** + Trunk, at parity with
the Leptos showcase: dataset-size / column-count / theme / language selectors, editable /
selectable / column-reorder toggles, and column-layout persistence. The grid is rendered
via the `rs_grid_dioxus::GridCanvas` component (a `key` bound to rows×cols remounts it on
dataset change). App logic: [`src/lib.rs`](src/lib.rs). Full build/run notes:
[`README.md`](README.md).

## Quick reference

```sh
trunk serve            # dev → http://localhost:9081 (hot-reload)
trunk build --release  # → dist/
```

## End-to-end tests (Playwright)

A functional + visual-regression suite ([`e2e/`](e2e/)) mirrors the Leptos demo and
covers every parity feature (selectors, toggles, persistence, canvas interaction).

```sh
cd e2e && npm install              # first time
npx playwright install chromium    # first time
cd .. && trunk build               # build dist/ first
cd e2e && npm test                 # run
cd e2e && npm run update-snapshots # regenerate visual baselines
```

## Critical: this repo does NOT contain the library

`rs-grid-*` and `example-common` are **git dependencies pinned to a tag** (currently
`rs-grid-core-v0.1.3`, see [`Cargo.toml`](Cargo.toml)):

- The library source is in the separate `rs-grid` repo. Editing files here changes only
  the demo wiring in `src/lib.rs` — never grid behaviour.
- **All deps must share the exact same tag.** Mixing per-crate tags breaks the build
  (`example-common` must match the library it was built against).
- To adopt a new library version: bump the tag on all deps together, then `cargo update`.

> **Temporary (pre-release dev):** `Cargo.toml` carries a `[patch."…/rs-grid"]` block
> pointing the `rs-grid-*` deps at a local working tree so the demo can build against
> unreleased API (`example_common::layout`, `rs_grid_web::storage`). Remove it and bump
> the `tag` once the new rs-grid version ships.

## Conventions

- `themes/` is **vendored** from the rs-grid reference theme — re-vendor rather than hand-edit.
- Rust files are auto-formatted on save (PostToolUse `rustfmt` hook). No clippy hook: this
  is a `cdylib` + `wasm-bindgen` crate, so host-target clippy does not apply.
- In `e2e/`, `node_modules/` and `test-results/` are gitignored; `tests/snapshots/` (visual
  baselines) are committed.
