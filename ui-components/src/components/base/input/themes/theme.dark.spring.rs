use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::{InputColors, InputTheme};

pub fn get_theme() -> InputTheme {
    ComponentTheme {
        name: "Dark Spring".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Spring),
        colors: InputColors {
            background: "green-900".to_string(),
            text: "green-100".to_string(),
            placeholder: "green-400".to_string(),
            border: "green-400/50".to_string(),
            initial_focus_ring: "green-400/50".to_string(),
            focus_ring: "green-400/50".to_string(),
            focus_border: "green-400".to_string(),
            hover_background: "green-800".to_string(),
            hover_border: "green-500".to_string(),
            disabled_background: "green-800".to_string(),
            disabled_text: "green-600".to_string(),
            disabled_border: "green-700".to_string(),
            selection_background: "green-700".to_string(),
            selection_text: "green-100".to_string(),
        },
    }
} 