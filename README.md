# rs-grid-example-dioxus

Full-featured example of [rs-grid](https://github.com/ruxelion/rs-grid) with
**Dioxus 0.7 (CSR)** and [Trunk](https://trunkrs.dev), at parity with the Leptos
showcase: dataset-size / column-count / theme / language selectors, editable /
selectable / column-reorder toggles, and column-layout persistence
(`localStorage`), over a virtual dataset.

This example pins the library at a released tag (`rs-grid-core-v0.1.3`) via a git
dependency — see [`Cargo.toml`](Cargo.toml). (A temporary `[patch]` block builds
against a local `rs-grid` working tree during pre-release development; it is
removed once the new tag ships.)

## Prerequisites

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
```

## Develop

```sh
trunk serve     # http://localhost:9081 (hot-reload)
```

## Build

```sh
trunk build --release   # → dist/
```

## End-to-end tests

A Playwright suite (functional + visual regression) in [`e2e/`](e2e/) covers
every feature — selectors, toggles, persistence, and canvas interaction.

```sh
cd e2e && npm install              # first time
npx playwright install chromium    # first time
cd .. && trunk build               # build dist/ first
cd e2e && npm test                 # run
cd e2e && npm run update-snapshots # regenerate visual baselines
```

The grid theme is defined by the CSS variables in [`themes/`](themes/) (vendored
from the rs-grid reference theme).
