use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::MainContentColors;

pub fn get_theme() -> ComponentTheme<MainContentColors> {
    ComponentTheme {
        name: "Light Summer".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Summer),
        colors: MainContentColors {
            background: "#fffaf0".to_string(),
            text: "#c05621".to_string(),
            text_muted: "#ed8936".to_string(),
            border: "#fbd38d".to_string(),
            shadow: "rgba(237, 137, 54, 0.1)".to_string(),
        },
    }
} 