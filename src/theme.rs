/// System theme detection with manual override support.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    System,
    Light,
    Dark,
}

impl ThemeMode {
    pub fn next(self) -> Self {
        match self {
            ThemeMode::System => ThemeMode::Light,
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::System,
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            ThemeMode::System => "💻",
            ThemeMode::Light => "☀",
            ThemeMode::Dark => "🌙",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ThemeMode::System => "System",
            ThemeMode::Light => "Light",
            ThemeMode::Dark => "Dark",
        }
    }
}

impl Theme {
    pub fn detect_system() -> Self {
        match dark_light::detect() {
            Ok(dark_light::Mode::Light) => Theme::Light,
            _ => Theme::Dark,
        }
    }

}
