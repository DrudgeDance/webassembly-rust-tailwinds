use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::{InputColors, InputTheme};

pub fn get_theme() -> InputTheme {
    ComponentTheme {
        name: "Light Summer".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Summer),
        colors: InputColors {
            background: "orange-50".to_string(),
            text: "orange-900".to_string(),
            placeholder: "orange-500".to_string(),
            border: "orange-500/50".to_string(),
            initial_focus_ring: "orange-500/50".to_string(),
            focus_ring: "orange-500/50".to_string(),
            focus_border: "orange-500".to_string(),
            hover_background: "orange-100".to_string(),
            hover_border: "orange-400".to_string(),
            disabled_background: "orange-50".to_string(),
            disabled_text: "orange-300".to_string(),
            disabled_border: "orange-200".to_string(),
            selection_background: "orange-200".to_string(),
            selection_text: "orange-900".to_string(),
        },
    }
} 