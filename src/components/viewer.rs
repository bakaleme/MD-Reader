use crate::{markdown::render_markdown, state::AppState};
use dioxus::prelude::*;
use std::time::Duration;

#[component]
pub fn Viewer() -> Element {
    let mut state = use_context::<AppState>();
    let mut file_text = use_signal(|| None::<Result<String, String>>);

    // Load file when selection changes
    use_effect(move || {
        let path = state.selected_file.read().clone();
        spawn(async move {
            match path {
                Some(p) => {
                    match tokio::fs::read_to_string(&p).await {
                        Ok(text) => {
                            if state.selected_file.read().as_ref() == Some(&p) {
                                file_text.set(Some(Ok(text)));
                            }
                        }
                        Err(e) => {
                            if state.selected_file.read().as_ref() == Some(&p) {
                                file_text.set(Some(Err(format!("Failed to read file: {e}"))));
                            }
                        }
                    }
                }
                None => file_text.set(None),
            }
        });
    });

    // Poll for external file changes
    use_hook(|| {
        spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(5)).await;
                if let Some(path) = state.selected_file.read().clone() {
                    if let Ok(text) = tokio::fs::read_to_string(&path).await {
                        if state.selected_file.read().as_ref() == Some(&path) {
                            file_text.set(Some(Ok(text)));
                        }
                    }
                }
            }
        });
    });

    let rendered = use_memo(move || {
        match file_text.read().as_ref() {
            None => None,
            Some(Ok(text)) => {
                if (state.view_raw)() {
                    Some(Ok((text.clone(), Vec::new())))
                } else {
                    let rendered = render_markdown(text);
                    Some(Ok((rendered.html, rendered.toc)))
                }
            }
            Some(Err(e)) => Some(Err(e.clone())),
        }
    });

    rsx! {
        div { class: "flex flex-col flex-1 h-full overflow-hidden",
            if let Some(ref path) = *state.selected_file.read() {
                div {
                    class: "flex items-center justify-between px-4 h-10 border-b shrink-0 bg-slate-50 dark:bg-slate-900 border-slate-200 dark:border-slate-800",
                    span { class: "text-sm font-mono opacity-60 truncate",
                        "{path.file_name().unwrap_or_default().to_string_lossy()}"
                    }
                    button {
                        class: "flex items-center justify-between gap-2 px-2 py-1 rounded text-xs font-medium transition-colors w-28 bg-slate-200 dark:bg-slate-700 text-slate-700 dark:text-slate-300 hover:bg-slate-300 dark:hover:bg-slate-600",
                        onclick: move |_| state.view_raw.toggle(),
                        span { class: "whitespace-nowrap", if (state.view_raw)() { "Raw" } else { "Preview" } }
                        div { class: "flex gap-0.5 opacity-60",
                            kbd { class: "px-1 rounded border font-mono text-[9px] bg-slate-100 dark:bg-slate-600 border-slate-300 dark:border-slate-500 text-slate-600 dark:text-slate-400", "Ctrl" }
                            kbd { class: "px-1 rounded border font-mono text-[9px] bg-slate-100 dark:bg-slate-600 border-slate-300 dark:border-slate-500 text-slate-600 dark:text-slate-400", "R" }
                        }
                    }
                }

                match rendered.read().as_ref() {
                    Some(Ok((html, toc))) if !html.is_empty() => rsx! {
                        div { class: "flex flex-1 overflow-hidden",
                            if !toc.is_empty() && !(state.view_raw)() && (state.toc_visible)() {
                                div {
                                    class: "hidden lg:flex flex-col w-56 shrink-0 overflow-y-auto border-l p-4 bg-slate-50 dark:bg-slate-900 border-slate-200 dark:border-slate-800",
                                    h4 { class: "text-xs font-semibold uppercase tracking-wider opacity-50 mb-3", "Contents" }
                                    ul { class: "space-y-1",
                                        for entry in toc.iter() {
                                            li {
                                                key: "{entry.id}",
                                                class: "text-sm",
                                                style: "padding-left: {(entry.level.saturating_sub(1) * 12)}px",
                                                a {
                                                    class: "block py-0.5 transition-colors text-slate-500 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white",
                                                    href: "#{entry.id}",
                                                    onclick: move |e| { e.prevent_default(); },
                                                    "{entry.text}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            div { class: "flex-1 overflow-y-auto",
                                if (state.view_raw)() {
                                    pre {
                                        class: "p-6 lg:p-10 xl:px-16 2xl:px-24 text-sm font-mono mx-auto max-w-5xl w-full bg-white dark:bg-slate-950 text-slate-800 dark:text-slate-300",
                                        "{html}"
                                    }
                                } else {
                                    div {
                                        class: "markdown-body p-6 lg:p-10 xl:px-16 2xl:px-24 mx-auto max-w-3xl xl:max-w-4xl 2xl:max-w-5xl",
                                        dangerous_inner_html: "{html}"
                                    }
                                }
                            }
                        }
                    },
                    Some(Ok(_)) => rsx! {
                        div { class: "flex-1 flex items-center justify-center",
                            p { class: "text-sm opacity-50", "File is empty." }
                        }
                    },
                    Some(Err(msg)) => rsx! {
                        div { class: "flex-1 flex items-center justify-center p-6",
                            div {
                                class: "px-4 py-3 rounded-md border bg-red-50 dark:bg-red-900/30 text-red-700 dark:text-red-300 border-red-200 dark:border-red-800",
                                p { "{msg}" }
                            }
                        }
                    },
                    None => rsx! {
                        div { class: "flex-1 flex flex-col items-center justify-center gap-3",
                            div { class: "w-8 h-8 border-2 border-blue-600 border-t-transparent rounded-full animate-spin" }
                            p { class: "text-sm opacity-50", "Loading..." }
                        }
                    },
                }
            } else {
                Welcome {}
            }
        }
    }
}

#[component]
fn Welcome() -> Element {
    let state = use_context::<AppState>();

    rsx! {
        div { class: "flex-1 flex flex-col items-center justify-center text-center p-8",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "64", height: "64", view_box: "0 0 24 24",
                fill: "none", stroke: "currentColor", stroke_width: "1.5",
                stroke_linecap: "round", stroke_linejoin: "round",
                class: "opacity-20 mb-6",
                path { d: "M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" }
                polyline { points: "14 2 14 8 20 8" }
                line { x1: "16", y1: "13", x2: "8", y2: "13" }
                line { x1: "16", y1: "17", x2: "8", y2: "17" }
                line { x1: "10", y1: "9", x2: "8", y2: "9" }
            }
            h1 { class: "text-4xl font-bold mb-2", "md-r" }
            p { class: "opacity-50 mb-8", "A lightweight Markdown reader built with Dioxus." }

            div { class: "grid grid-cols-2 gap-3 text-xs mb-8 max-w-sm",
                Shortcut { keys: &["Ctrl", "O"], desc: "Open folder" }
                Shortcut { keys: &["Ctrl", "R"], desc: "Raw / Preview" }
                Shortcut { keys: &["Ctrl", "T"], desc: "Toggle theme" }
                Shortcut { keys: &["Ctrl", "B"], desc: "Toggle sidebar" }
                Shortcut { keys: &["Ctrl", "Shift", "B"], desc: "Toggle TOC" }
            }

            button {
                class: "px-4 py-2 rounded-md bg-blue-600 text-white text-sm font-medium hover:bg-blue-700 transition-colors",
                onclick: move |_| state.open_directory(),
                "Open a Folder"
            }
        }
    }
}

#[component]
fn Shortcut(keys: &'static [&'static str], desc: &'static str) -> Element {
    rsx! {
        div { class: "flex items-center gap-2",
            div { class: "flex gap-1",
                for key in keys.iter() {
                    kbd {
                        key: "{key}",
                        class: "px-1.5 py-0.5 rounded border font-mono text-[10px] bg-slate-100 dark:bg-slate-800 border-slate-300 dark:border-slate-700 text-slate-600 dark:text-slate-300",
                        "{key}"
                    }
                }
            }
            span { class: "opacity-50", "{desc}" }
        }
    }
}
