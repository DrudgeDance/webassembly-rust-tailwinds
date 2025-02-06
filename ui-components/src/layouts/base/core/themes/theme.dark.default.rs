use crate::theme::{ComponentTheme, Mode};
use super::types::LayoutColors;

pub fn get_theme() -> ComponentTheme<LayoutColors> {
    ComponentTheme {
        name: "Dark Default".to_string(),
        mode: Mode::Dark,
        theme: None,
        colors: LayoutColors {
            background: "gray-900".to_string(),
            surface: "gray-800".to_string(),
            text: "gray-100".to_string(),
            text_muted: "gray-400".to_string(),
            border: "gray-600".to_string(),
            shadow: "gray-700".to_string(),
        },
    }
} 