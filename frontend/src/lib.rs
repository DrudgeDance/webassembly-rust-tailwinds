pub mod features;
pub mod services;
pub mod providers;

use leptos::*;
use crate::features::hello::HelloFeature;
use crate::providers::theme::ThemeProvider;
use ui_components::theme::{BaseTheme, Mode, ThemeColors};

#[component]
pub fn App() -> impl IntoView {
    let (theme, set_theme) = create_signal(BaseTheme {
        mode: Mode::Light,
        theme: None,
        name: "Default".to_string(),
        colors: ThemeColors {
            background: "#ffffff".to_string(),
            surface: "#f5f5f5".to_string(),
            text: "#000000".to_string(),
            text_muted: "#666666".to_string(),
            primary: "#1a73e8".to_string(),
            secondary: "#9c27b0".to_string(),
            accent: "#ff4081".to_string(),
        },
    });

    view! {
        <ThemeProvider theme=theme set_theme=set_theme>
            <HelloFeature theme=theme />
        </ThemeProvider>
    }
} 