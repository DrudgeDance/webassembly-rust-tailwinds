use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::FootbarColors;

pub fn get_theme() -> ComponentTheme<FootbarColors> {
    ComponentTheme {
        name: "Light Summer".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Summer),
        colors: FootbarColors {
            background: "#fffaf0".to_string(),
            surface: "#fffaf0".to_string(),
            text: "#7b341e".to_string(),
            text_muted: "#ed8936".to_string(),
            border: "#fbd38d".to_string(),
        },
    }
} 