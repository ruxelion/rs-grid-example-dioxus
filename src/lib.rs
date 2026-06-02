//! Demo application showcasing rs-grid with Dioxus 0.6 web.

use std::{cell::RefCell, rc::Rc};

use dioxus::prelude::*;
use example_common::{build_model, fmt_cols, fmt_rows};
use rs_grid_core::state::GridState;
use rs_grid_dioxus::{theme_from_css_vars, Locale, WebGridCanvas};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlCanvasElement;

// ── Types ──────────────────────────────────────────────────────────────────

type CanvasRef = Rc<RefCell<Option<HtmlCanvasElement>>>;
type GridRef = Rc<RefCell<Option<WebGridCanvas>>>;

// ── remount ────────────────────────────────────────────────────────────────

/// Detach the current GridCanvas if any and mount a fresh one with
/// `rows` × `cols` of virtual data.
fn remount(canvas_ref: &CanvasRef, grid_ref: &GridRef, rows: u64, cols: usize) {
    let Some(canvas) = canvas_ref.borrow().clone() else {
        return;
    };
    if let Some(old) = grid_ref.borrow().as_ref() {
        old.detach();
    }
    let model = build_model(rows, cols);
    let w = canvas.client_width() as f64;
    let h = canvas.client_height() as f64;
    let state = GridState::new(model, w, h);
    let gc = WebGridCanvas::mount(
        canvas,
        state,
        theme_from_css_vars(),
        Locale::default(),
    );
    gc.render();
    *grid_ref.borrow_mut() = Some(gc);
}

// ── App component ──────────────────────────────────────────────────────────

#[component]
fn App() -> Element {
    let mut row_count = use_signal(|| 1_000u64);
    let mut col_count = use_signal(|| 20usize);
    let mut theme_class = use_signal(String::new);

    let canvas_ref: CanvasRef =
        use_hook(|| Rc::new(RefCell::new(None::<HtmlCanvasElement>))).clone();
    let grid_ref: GridRef =
        use_hook(|| Rc::new(RefCell::new(None::<WebGridCanvas>))).clone();

    let cr_mount = Rc::clone(&canvas_ref);
    let gr_mount = Rc::clone(&grid_ref);
    let cr_rows = Rc::clone(&canvas_ref);
    let gr_rows = Rc::clone(&grid_ref);
    let cr_cols = Rc::clone(&canvas_ref);
    let gr_cols = Rc::clone(&grid_ref);
    let gr_theme = Rc::clone(&grid_ref);

    use_effect(move || {
        let cls = theme_class.read().clone();
        if let Some(root) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.document_element())
        {
            root.set_class_name(&cls);
        }
        if let Some(gc) = gr_theme.borrow().as_ref() {
            gc.set_theme(theme_from_css_vars());
        }
    });

    rsx! {
        main { class: "app-layout",
            div { class: "app-page-header",
                h1 { class: "app-title", "rs-grid basic example" }
                p { class: "app-subtitle",
                    "Use the "
                    strong { class: "app-highlight",
                        { fmt_rows(*row_count.read()) }
                    }
                    " × "
                    strong { class: "app-highlight",
                        { fmt_cols(*col_count.read()) }
                    }
                    " virtual dataset below to test windowed rendering."
                }
                div { class: "app-controls",

                    // ── Dataset size ──────────────────────────────────
                    div { class: "app-control",
                        span { class: "app-control-label", "Dataset size" }
                        select {
                            class: "app-control-select",
                            onchange: move |e| {
                                let v = e.value()
                                    .parse::<u64>()
                                    .unwrap_or(1_000);
                                row_count.set(v);
                                remount(
                                    &cr_rows,
                                    &gr_rows,
                                    v,
                                    *col_count.peek(),
                                );
                            },
                            option { value: "1000",             "1 000 rows" }
                            option { value: "100000",           "100 000 rows" }
                            option { value: "1000000",          "1 million rows" }
                            option { value: "100000000",        "100 million rows" }
                            option { value: "1000000000",       "1 billion rows" }
                            option { value: "1000000000000",    "1 trillion rows" }
                            option {
                                value: "1000000000000000",
                                "1 quadrillion rows"
                            }
                        }
                    }

                    // ── Column count ──────────────────────────────────
                    div { class: "app-control",
                        span { class: "app-control-label", "Column count" }
                        select {
                            class: "app-control-select",
                            onchange: move |e| {
                                let v = e.value()
                                    .parse::<usize>()
                                    .unwrap_or(20);
                                col_count.set(v);
                                remount(
                                    &cr_cols,
                                    &gr_cols,
                                    *row_count.peek(),
                                    v,
                                );
                            },
                            option { value: "20",   "20 columns" }
                            option { value: "100",  "100 columns" }
                            option { value: "1000", "1 000 columns" }
                        }
                    }

                    // ── Theme ─────────────────────────────────────────
                    div { class: "app-control",
                        span { class: "app-control-label", "Theme" }
                        select {
                            class: "app-control-select",
                            onchange: move |e| {
                                theme_class.set(e.value());
                            },
                            option { value: "",       "Light" }
                            option { value: "dark",   "Dark" }
                            option { value: "dimmed", "Dimmed" }
                        }
                    }
                }
            }

            // ── Body: grid canvas ─────────────────────────────────────
            div { class: "app-body",
                div { class: "app-grid-wrapper",
                    canvas {
                        id: "rs-grid-canvas",
                        style: "width:100%;height:100%;display:block",
                        onmounted: move |_| {
                            if let Some(canvas) = web_sys::window()
                                .and_then(|w| w.document())
                                .and_then(|d| {
                                    d.get_element_by_id("rs-grid-canvas")
                                })
                                .and_then(|el| {
                                    el.dyn_into::<HtmlCanvasElement>().ok()
                                })
                            {
                                *cr_mount.borrow_mut() = Some(canvas);
                                remount(
                                    &cr_mount,
                                    &gr_mount,
                                    *row_count.peek(),
                                    *col_count.peek(),
                                );
                            }
                        },
                    }
                }
            }
        }
    }
}

// ── WASM entry point ───────────────────────────────────────────────────────

/// WASM entry point — mount the Dioxus app.
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    dioxus::launch(App);
}
