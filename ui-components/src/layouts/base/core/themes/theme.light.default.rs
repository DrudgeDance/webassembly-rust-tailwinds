use crate::theme::{ComponentTheme, Mode};
use super::types::LayoutColors;

pub fn get_theme() -> ComponentTheme<LayoutColors> {
    ComponentTheme {
        name: "Light Default".to_string(),
        mode: Mode::Light,
        theme: None,
        colors: LayoutColors {
            background: "gray-50".to_string(),
            surface: "white".to_string(),
            text: "gray-900".to_string(),
            text_muted: "gray-600".to_string(),
            border: "gray-200".to_string(),
            shadow: "gray-200".to_string(),
        },
    }
} 