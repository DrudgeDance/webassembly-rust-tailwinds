use crate::theme::{ComponentTheme, Mode};
use super::types::HelloColors;

pub fn get_theme() -> ComponentTheme<HelloColors> {
    ComponentTheme {
        name: "Dark Default".to_string(),
        mode: Mode::Dark,
        theme: None,
        colors: HelloColors {
            background: "gray-900".to_string(),
            surface: "gray-800".to_string(),
            text: "white".to_string(),
            text_muted: "gray-300".to_string(),
            border: "gray-500".to_string(),
            border_hover: "gray-400".to_string(),
            shadow_color: "gray-900".to_string(),
            hover_background: "gray-700".to_string(),
            hover_border: "gray-400".to_string(),
            selection_background: "gray-700".to_string(),
            selection_text: "white".to_string(),
        },
    }
} 