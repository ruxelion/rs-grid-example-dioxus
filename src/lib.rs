//! Demo application showcasing rs-grid with Dioxus 0.7 web.

use std::{cell::RefCell, rc::Rc};

use dioxus::prelude::*;
use example_common::{
    build_model, class_map::resolve_classes, fmt_cols, fmt_rows,
    layout::LayoutSnapshot,
};
use rs_grid_dioxus::{
    theme_from_css_vars, GridCanvas, Locale, ModelSlot, WebGridCanvas,
};
use rs_grid_web::storage;
use wasm_bindgen::prelude::*;

/// localStorage key for the persisted column layout.
const LS_KEY: &str = "rs-grid-basic-layout";

/// Live handle to the mounted grid, shared with the toggle effects.
type GridRef = Rc<RefCell<Option<WebGridCanvas>>>;

/// Detect the initial language code from the browser, restricted to the
/// primary subtag (falls back to English).
fn initial_lang_code() -> String {
    web_sys::window()
        .and_then(|w| w.navigator().language())
        .unwrap_or_default()
        .split('-')
        .next()
        .unwrap_or("en")
        .to_string()
}

/// Apply a CSS class to the document root element (theme switch).
fn set_root_class(cls: &str) {
    if let Some(root) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
    {
        root.set_class_name(cls);
    }
}

// ── App component ────────────────────────────────────────────────────────────

#[component]
fn App() -> Element {
    let mut row_count = use_signal(|| 1_000u64);
    let mut col_count = use_signal(|| 20usize);
    let mut theme_class = use_signal(String::new);
    let mut editable = use_signal(|| true);
    let mut selectable = use_signal(|| true);
    let mut column_reorderable = use_signal(|| true);
    let mut lang_code = use_signal(initial_lang_code);
    let locale = use_signal(Locale::from_browser);
    let mut validation_error = use_signal(String::new);
    let last_button_action = use_signal(String::new);

    // Live canvas handle, populated by on_mount and read by the toggle effects.
    let grid_ref: GridRef = use_hook(|| Rc::new(RefCell::new(None))).clone();

    // Theme: apply the root class, then repaint the grid in the new theme
    // (CSS vars are read AFTER the class is applied).
    {
        let grid_ref = grid_ref.clone();
        use_effect(move || {
            let cls = theme_class.read().clone();
            set_root_class(&cls);
            if let Some(gc) = grid_ref.borrow().as_ref() {
                gc.set_theme(theme_from_css_vars());
            }
        });
    }

    // Editable toggle → live canvas.
    {
        let grid_ref = grid_ref.clone();
        use_effect(move || {
            let v = editable();
            if let Some(gc) = grid_ref.borrow().as_ref() {
                gc.set_editable(v);
            }
        });
    }
    // Selectable toggle → live canvas.
    {
        let grid_ref = grid_ref.clone();
        use_effect(move || {
            let v = selectable();
            if let Some(gc) = grid_ref.borrow().as_ref() {
                gc.set_selectable(v);
            }
        });
    }
    // Column reorder toggle → live canvas.
    {
        let grid_ref = grid_ref.clone();
        use_effect(move || {
            let v = column_reorderable();
            if let Some(gc) = grid_ref.borrow().as_ref() {
                gc.set_column_reorderable(v);
            }
        });
    }

    // Build the model for the current dataset, with persisted layout applied
    // before mount. Changing rows/cols changes `grid_key`, remounting the grid.
    let mut model = build_model(row_count(), col_count());
    if let Some(raw) = storage::get_item(LS_KEY) {
        if let Some(snapshot) = LayoutSnapshot::from_json(&raw) {
            snapshot.apply(&mut model);
        }
    }
    let grid_key = format!("{}-{}", row_count(), col_count());

    // on_mount: wire resolver / initial toggles / persistence / button click.
    let gr_mount = grid_ref.clone();
    let on_mount = move |gc: WebGridCanvas| {
        gc.set_class_resolver(Rc::new(resolve_classes));
        gc.set_editable(*editable.peek());
        gc.set_selectable(*selectable.peek());
        gc.set_column_reorderable(*column_reorderable.peek());

        // Persist column layout so user resizes / reorders survive a reload.
        let gc_save = gc.clone();
        gc.set_on_columns_changed(move || {
            let snapshot = LayoutSnapshot::new(
                gc_save.column_widths(),
                gc_save.column_order(),
                gc_save.pinned_count(),
            );
            if let Some(json) = snapshot.to_json() {
                storage::set_item(LS_KEY, &json);
            }
        });

        // Cell button clicks → status line. `set_on_cell_button_click` takes
        // an `Fn` closure, so rebind the (Copy) signal as a mutable local.
        gc.set_on_cell_button_click(move |row, col, btn| {
            let mut last_button_action = last_button_action;
            last_button_action.set(format!("[{btn}] row={row} col={col}"));
        });

        *gr_mount.borrow_mut() = Some(gc);
    };

    rsx! {
        main { class: "app-layout",
            div { class: "app-page-header",
                h1 { class: "app-title", "rs-grid basic example" }
                p { class: "app-subtitle",
                    "Use the "
                    strong { class: "app-highlight", { fmt_rows(row_count()) } }
                    " × "
                    strong { class: "app-highlight", { fmt_cols(col_count()) } }
                    " virtual dataset below to test windowed rendering."
                }
                div { class: "app-controls",

                    // ── Dataset size ──────────────────────────────────
                    div { class: "app-control",
                        span { class: "app-control-label", "Dataset size" }
                        select {
                            class: "app-control-select",
                            onchange: move |e| {
                                row_count.set(
                                    e.value().parse::<u64>().unwrap_or(1_000),
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
                                col_count.set(
                                    e.value().parse::<usize>().unwrap_or(20),
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
                            onchange: move |e| { theme_class.set(e.value()); },
                            option { value: "",       "Light" }
                            option { value: "dark",   "Dark" }
                            option { value: "dimmed", "Dimmed" }
                        }
                    }

                    // ── Language ──────────────────────────────────────
                    div { class: "app-control",
                        span { class: "app-control-label", "Language" }
                        select {
                            class: "app-control-select",
                            value: "{lang_code}",
                            onchange: move |e| {
                                let v = e.value();
                                lang_code.set(v.clone());
                                let mut locale = locale;
                                locale.set(Locale::from_language_tag(&v));
                            },
                            option { value: "en", "English" }
                            option { value: "fr", "Fran\u{e7}ais" }
                            option { value: "de", "Deutsch" }
                            option { value: "es", "Espa\u{f1}ol" }
                            option { value: "it", "Italiano" }
                            option { value: "pt", "Portugu\u{ea}s" }
                            option { value: "nl", "Nederlands" }
                            option { value: "pl", "Polski" }
                            option { value: "tr", "T\u{fc}rk\u{e7}e" }
                            option { value: "ru", "Русский" }
                            option { value: "uk", "Українська" }
                            option { value: "ar", "العربية" }
                            option { value: "ja", "日本語" }
                            option { value: "zh", "中文" }
                            option { value: "ko", "한국어" }
                        }
                    }

                    // ── Editable toggle ───────────────────────────────
                    div { class: "app-control",
                        span { class: "app-control-label", "Editable" }
                        label { class: "app-switch",
                            input {
                                r#type: "checkbox",
                                checked: editable(),
                                onchange: move |e| { editable.set(e.checked()); },
                            }
                            span { class: "app-switch-track" }
                        }
                    }

                    // ── Selectable toggle ─────────────────────────────
                    div { class: "app-control",
                        span { class: "app-control-label", "Selectable" }
                        label { class: "app-switch",
                            input {
                                r#type: "checkbox",
                                checked: selectable(),
                                onchange: move |e| {
                                    selectable.set(e.checked());
                                },
                            }
                            span { class: "app-switch-track" }
                        }
                    }

                    // ── Column reorder toggle ─────────────────────────
                    div { class: "app-control",
                        span { class: "app-control-label", "Column reorder" }
                        label { class: "app-switch",
                            input {
                                r#type: "checkbox",
                                checked: column_reorderable(),
                                onchange: move |e| {
                                    column_reorderable.set(e.checked());
                                },
                            }
                            span { class: "app-switch-track" }
                        }
                    }

                    // ── Reset persisted layout ────────────────────────
                    div { class: "app-control",
                        span { class: "app-control-label", "Layout" }
                        button {
                            class: "app-control-button",
                            onclick: move |_| {
                                storage::remove_item(LS_KEY);
                                if let Some(w) = web_sys::window() {
                                    let _ = w.location().reload();
                                }
                            },
                            "Reset"
                        }
                    }
                }
            }

            // ── Validation error display ──────────────────────────────
            if !validation_error().is_empty() {
                div { class: "app-validation-error", { validation_error() } }
            }

            // ── Cell button click display ─────────────────────────────
            if !last_button_action().is_empty() {
                div { class: "app-validation-error",
                    "Button clicked: "
                    { last_button_action() }
                }
            }

            // ── Body: grid canvas ─────────────────────────────────────
            div { class: "app-body",
                div { class: "app-grid-wrapper",
                    GridCanvas {
                        key: "{grid_key}",
                        model: ModelSlot::new(model),
                        width: "100%",
                        height: "100%",
                        locale,
                        on_mount,
                        on_validation_error: move |evt: (u64, String, String)| {
                            validation_error.set(format!("[{}] {}", evt.1, evt.2));
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
