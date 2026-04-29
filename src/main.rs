#![allow(non_snake_case)]

mod app;
mod components;
mod markdown;
mod state;
mod theme;

fn main() {
    #[cfg(debug_assertions)]
    {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("info")
            .try_init();
    }

    #[cfg(feature = "desktop")]
    {
        use dioxus::desktop::{Config, WindowBuilder, LogicalSize};

        dioxus::LaunchBuilder::desktop()
            .with_cfg(
                Config::new().with_window(
                    WindowBuilder::new()
                        .with_title("md-r")
                        .with_decorations(false)
                        .with_inner_size(LogicalSize::new(1200.0, 800.0))
                        .with_min_inner_size(LogicalSize::new(640.0, 480.0)),
                ),
            )
            .launch(app::App);
    }

    #[cfg(not(feature = "desktop"))]
    {
        dioxus::launch(app::App);
    }
}
