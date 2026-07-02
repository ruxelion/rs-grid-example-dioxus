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

## Before coding

<!-- keep in sync with the "Before coding" section in every other repo's AGENTS.md -->
**Plan before coding non-trivial changes.** For a bug fix or feature that
touches more than one file, changes a public API/component contract, or
isn't an obvious one-liner, propose a short plan (approach, files touched,
trade-offs) before writing code — use Claude Code's Plan Mode rather than
diving straight into edits. Skip this for trivial fixes; planning every
one-line change only adds friction.

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

CI (`.github/workflows/ci.yml`, `e2e` job) runs the full suite automatically
on every push/PR with `--update-snapshots --grep-invert "visual regression"` —
screenshots always write fresh rather than diff against a baseline (pixel
comparison across CI/dev environments is unreliable), so only the functional
assertions actually gate the build.

## Critical: this repo does NOT contain the library

<!-- keep in sync with rs-grid/AGENTS.md "How they relate" + the other 3
     rs-grid-example-*/AGENTS.md "Critical" sections -->

`rs-grid-*` and `example-common` are **git dependencies pinned to a tag** — see
the `tag =` value in [`Cargo.toml`](Cargo.toml) for the current pin (do not
hardcode a version/tag name in prose here, it goes stale):

- The library source is in the separate `rs-grid` repo. Editing files here changes only
  the demo wiring in `src/lib.rs` — never grid behaviour.
- **All deps must share the exact same tag.** Mixing per-crate tags breaks the build
  (`example-common` must match the library it was built against).
- To adopt a new library version: bump the tag on all deps together, then `cargo update`.

> **Temporary (pre-release dev) pattern:** if `Cargo.toml` carries a
> `[patch."…/rs-grid"]` block, it points the `rs-grid-*` deps at a local working
> tree so the demo can build against unreleased API before a version ships.
> Remove it and bump the `tag` once the new rs-grid version ships. (No patch
> block is active right now — check `Cargo.toml` before assuming one exists.)

## Conventions

- `themes/` is **vendored** from the rs-grid reference theme — re-vendor rather than hand-edit.
- Rust files are auto-formatted on save (PostToolUse `rustfmt` hook, then a blocking
  `cargo check --target wasm32-unknown-unknown`). No clippy hook: this is a `cdylib` +
  `wasm-bindgen` crate, so host-target clippy does not apply.
- Formatting uses stable `rustfmt` defaults (no `rustfmt.toml` here, unlike `rs-grid`'s
  nightly-only config) — intentional, so this demo never requires a nightly toolchain.
- No `unwrap()` in production code — use `expect("reason")` or error propagation.
- English (US) only in code, comments, and strings.
- In `e2e/`, `node_modules/` and `test-results/` are gitignored; `tests/snapshots/` (visual
  baselines) are committed.

## Public surface (keep in sync with `README.md`)

The selectors/toggles wired to `rs_grid_dioxus::GridCanvas` in
[`src/lib.rs`](src/lib.rs) (dataset size, column count, theme, language,
editable/selectable/column-reorder, layout persistence) are this demo's
public-facing contract. If you add, remove, or rename one, update
`README.md`'s feature list in the same commit.
