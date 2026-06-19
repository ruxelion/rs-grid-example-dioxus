# rs-grid-example-dioxus

Standalone example of [rs-grid](https://github.com/ruxelion/rs-grid) with
**Dioxus 0.7 (CSR)** and [Trunk](https://trunkrs.dev). Row/column-count and
theme selectors over a virtual dataset.

This example pins the library at a released tag (`rs-grid-core-v0.1.3`) via a git
dependency — see [`Cargo.toml`](Cargo.toml).

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

The grid theme is defined by the CSS variables in [`themes/`](themes/) (vendored
from the rs-grid reference theme).
