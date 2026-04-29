use dioxus::prelude::*;
use std::path::PathBuf;

#[derive(Clone, Copy)]
pub struct AppState {
    pub current_dir: Signal<Option<PathBuf>>,
    pub selected_file: Signal<Option<PathBuf>>,
    pub search_query: Signal<String>,
    pub view_raw: Signal<bool>,
    pub theme_mode: Signal<crate::theme::ThemeMode>,
    pub sidebar_visible: Signal<bool>,
    pub toc_visible: Signal<bool>,
    pub is_dark: ReadSignal<bool>,
}

impl AppState {
    pub fn new() -> Self {
        let theme_mode = use_signal(|| crate::theme::ThemeMode::System);
        let system_theme = crate::theme::Theme::detect_system();

        let is_dark = use_memo(move || {
            match theme_mode() {
                crate::theme::ThemeMode::Light => false,
                crate::theme::ThemeMode::Dark => true,
                crate::theme::ThemeMode::System => system_theme == crate::theme::Theme::Dark,
            }
        });

        Self {
            current_dir: use_signal(|| None),
            selected_file: use_signal(|| None),
            search_query: use_signal(String::new),
            view_raw: use_signal(|| false),
            theme_mode,
            sidebar_visible: use_signal(|| true),
            toc_visible: use_signal(|| true),
            is_dark: is_dark.into(),
        }
    }

    pub fn open_directory(mut self) {
        spawn(async move {
            if let Some(handle) = rfd::AsyncFileDialog::new().pick_folder().await {
                let path = handle.path().to_path_buf();
                self.current_dir.set(Some(path));
                self.selected_file.set(None);
                self.search_query.set(String::new());
                self.sidebar_visible.set(true);
            }
        });
    }

    pub fn select_file(mut self, path: PathBuf) {
        self.selected_file.set(Some(path));
    }
}
