use crate::theme::{ComponentTheme, Mode};
use super::types::{InputColors, InputTheme};

pub fn get_theme() -> InputTheme {
    ComponentTheme {
        name: "Dark Default".to_string(),
        mode: Mode::Dark,
        theme: None,
        colors: InputColors {
            background: "gray-800".to_string(),
            text: "gray-100".to_string(),
            placeholder: "gray-400".to_string(),
            border: "blue-400/50".to_string(),
            initial_focus_ring: "blue-400/50".to_string(),
            focus_ring: "blue-400/50".to_string(),
            focus_border: "blue-400".to_string(),
            hover_background: "gray-700".to_string(),
            hover_border: "blue-500".to_string(),
            disabled_background: "gray-700".to_string(),
            disabled_text: "gray-600".to_string(),
            disabled_border: "gray-700".to_string(),
            selection_background: "blue-900".to_string(),
            selection_text: "blue-100".to_string(),
        },
    }
} 