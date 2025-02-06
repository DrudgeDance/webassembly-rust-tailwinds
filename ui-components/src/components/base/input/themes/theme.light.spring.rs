use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::{InputColors, InputTheme};

pub fn get_theme() -> InputTheme {
    ComponentTheme {
        name: "Light Spring".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Spring),
        colors: InputColors {
            background: "green-50".to_string(),
            text: "green-900".to_string(),
            placeholder: "green-500".to_string(),
            border: "green-500/50".to_string(),
            initial_focus_ring: "green-500/50".to_string(),
            focus_ring: "green-500/50".to_string(),
            focus_border: "green-500".to_string(),
            hover_background: "green-100".to_string(),
            hover_border: "green-400".to_string(),
            disabled_background: "green-50".to_string(),
            disabled_text: "green-300".to_string(),
            disabled_border: "green-200".to_string(),
            selection_background: "green-200".to_string(),
            selection_text: "green-900".to_string(),
        },
    }
} 