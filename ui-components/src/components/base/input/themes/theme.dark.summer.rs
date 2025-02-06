use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::{InputColors, InputTheme};

pub fn get_theme() -> InputTheme {
    ComponentTheme {
        name: "Dark Summer".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Summer),
        colors: InputColors {
            background: "orange-900".to_string(),
            text: "orange-100".to_string(),
            placeholder: "orange-400".to_string(),
            border: "orange-400/50".to_string(),
            initial_focus_ring: "orange-400/50".to_string(),
            focus_ring: "orange-400/50".to_string(),
            focus_border: "orange-400".to_string(),
            hover_background: "orange-800".to_string(),
            hover_border: "orange-500".to_string(),
            disabled_background: "orange-800".to_string(),
            disabled_text: "orange-600".to_string(),
            disabled_border: "orange-700".to_string(),
            selection_background: "orange-700".to_string(),
            selection_text: "orange-100".to_string(),
        },
    }
} 