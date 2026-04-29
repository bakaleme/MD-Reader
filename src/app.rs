use crate::{components, state::AppState};
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let state = AppState::new();
    use_context_provider(|| state);

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        script { src: asset!("/assets/copy.js") }
        div {
            class: "flex flex-col h-screen w-screen overflow-hidden outline-none",
            class: if (state.is_dark)() { "dark bg-slate-950 text-slate-100" } else { "bg-white text-slate-900" },
            tabindex: "0",
            onkeydown: move |evt| handle_keydown(evt, state),
            div { class: "flex flex-1 overflow-hidden",
                if (state.sidebar_visible)() {
                    components::Sidebar {}
                }
                components::Viewer {}
            }
            StatusBar {}
        }
    }
}

fn handle_keydown(evt: Event<KeyboardData>, mut state: AppState) {
    use dioxus::html::input_data::keyboard_types::{Key, Modifiers};

    let mods = evt.modifiers();
    let key = evt.key();

    if !mods.contains(Modifiers::CONTROL) || mods.contains(Modifiers::ALT) || mods.contains(Modifiers::META) {
        return;
    }

    let has_shift = mods.contains(Modifiers::SHIFT);
    let is_char = |ch: &str| matches!(key, Key::Character(ref s) if s.eq_ignore_ascii_case(ch));

    match (has_shift, is_char("o"), is_char("r"), is_char("t"), is_char("b")) {
        (false, true, _, _, _) => { evt.prevent_default(); state.open_directory(); }
        (false, _, true, _, _) => { evt.prevent_default(); state.view_raw.toggle(); }
        (false, _, _, true, _) => { evt.prevent_default(); state.theme_mode.with_mut(|m| *m = m.next()); }
        (false, _, _, _, true) => { evt.prevent_default(); state.sidebar_visible.toggle(); }
        (true,  _, _, _, true) => { evt.prevent_default(); state.toc_visible.toggle(); }
        _ => {}
    }
}

#[component]
fn StatusBar() -> Element {
    let mut state = use_context::<AppState>();

    rsx! {
        div {
            class: "shrink-0 flex items-center gap-4 px-4 py-1.5 text-[11px] border-t select-none bg-slate-50 dark:bg-slate-950 text-slate-500 dark:text-slate-400 border-slate-200 dark:border-slate-800",
            if (state.current_dir)().is_some() {
                button {
                    class: "flex items-center px-2 py-1 rounded bg-red-600 text-white hover:bg-red-700 transition-colors text-[11px] font-medium",
                    onclick: move |_| {
                        state.current_dir.set(None);
                        state.selected_file.set(None);
                        state.search_query.set(String::new());
                    },
                    "<"
                }
            }
            button {
                class: "flex items-center gap-1.5 px-2 py-1 rounded bg-blue-600 text-white hover:bg-blue-700 transition-colors text-[11px] font-medium",
                onclick: move |_| state.open_directory(),
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "12", height: "12", view_box: "0 0 24 24",
                    fill: "none", stroke: "currentColor", stroke_width: "2",
                    stroke_linecap: "round", stroke_linejoin: "round",
                    path { d: "M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2z" }
                }
                "Open"
                div { class: "flex gap-0.5 opacity-80 ml-1",
                    kbd { class: "px-1 rounded border font-mono text-[9px] bg-blue-500 border-blue-400", "Ctrl" }
                    kbd { class: "px-1 rounded border font-mono text-[9px] bg-blue-500 border-blue-400", "O" }
                }
            }
            div { class: "w-px h-3 bg-slate-300 dark:bg-slate-700" }
            ShortcutHint { keys: &["Ctrl", "B"], desc: "Sidebar" }
            ShortcutHint { keys: &["Ctrl", "Shift", "B"], desc: "TOC" }
            div { class: "ml-auto flex items-center gap-3",
                button {
                    class: "px-2 py-1 rounded border text-[11px] font-medium transition-colors bg-slate-200 dark:bg-slate-800 border-slate-300 dark:border-slate-700 hover:bg-slate-300 dark:hover:bg-slate-700",
                    title: "Toggle theme (Ctrl+T)",
                    onclick: move |_| {
                        state.theme_mode.with_mut(|m| *m = m.next());
                    },
                    "{(state.theme_mode)().icon()} {(state.theme_mode)().label()}"
                }
                span { class: "opacity-60", "md-r v0.1.0" }
            }
        }
    }
}

#[component]
fn ShortcutHint(keys: &'static [&'static str], desc: &'static str) -> Element {
    rsx! {
        div { class: "flex items-center gap-1.5",
            div { class: "flex gap-1",
                for key in keys.iter() {
                    kbd {
                        key: "{key}",
                        class: "px-1 rounded border font-mono text-[10px] bg-slate-200 dark:bg-slate-800 border-slate-300 dark:border-slate-700",
                        "{key}"
                    }
                }
            }
            span { "{desc}" }
        }
    }
}
