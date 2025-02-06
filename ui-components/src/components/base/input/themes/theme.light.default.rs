use crate::theme::{ComponentTheme, Mode};
use super::types::{InputColors, InputTheme};

pub fn get_theme() -> InputTheme {
    ComponentTheme {
        name: "Light Default".to_string(),
        mode: Mode::Light,
        theme: None,
        colors: InputColors {
            background: "white".to_string(),
            text: "gray-900".to_string(),
            placeholder: "gray-500".to_string(),
            border: "blue-500/50".to_string(),
            initial_focus_ring: "blue-500/50".to_string(),
            focus_ring: "blue-500/50".to_string(),
            focus_border: "blue-500".to_string(),
            hover_background: "gray-50".to_string(),
            hover_border: "blue-400".to_string(),
            disabled_background: "gray-50".to_string(),
            disabled_text: "gray-300".to_string(),
            disabled_border: "gray-200".to_string(),
            selection_background: "blue-100".to_string(),
            selection_text: "blue-900".to_string(),
        },
    }
} 