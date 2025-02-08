use crate::theme::{ComponentTheme, Mode};
use super::types::HelloColors;

pub fn get_theme() -> ComponentTheme<HelloColors> {
    ComponentTheme {
        name: "Light Default".to_string(),
        mode: Mode::Light,
        theme: None,
        colors: HelloColors {
            background: "gray-50".to_string(),
            surface: "white".to_string(),
            text: "gray-900".to_string(),
            text_muted: "gray-600".to_string(),
            border: "gray-200".to_string(),
            border_hover: "gray-300".to_string(),
            shadow_color: "gray-200".to_string(),
            hover_background: "gray-50".to_string(),
            hover_border: "gray-300".to_string(),
            selection_background: "gray-100".to_string(),
            selection_text: "gray-900".to_string(),
        },
    }
} 