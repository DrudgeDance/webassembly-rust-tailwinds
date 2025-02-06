use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::{InputColors, InputTheme};

pub fn get_theme() -> InputTheme {
    ComponentTheme {
        name: "Light Spring".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Spring),
        colors: InputColors {
            background: "#f0fff4".to_string(),
            text: "#22543d".to_string(),
            placeholder: "#48bb78".to_string(),
            border: "#9ae6b4".to_string(),
            initial_focus_ring: "#48bb78".to_string(),
            focus_ring: "#48bb78".to_string(),
            focus_border: "#48bb78".to_string(),
            hover_background: "#e6ffec".to_string(),
            hover_border: "#4ade80".to_string(),
            disabled_background: "#e6ffec".to_string(),
            disabled_text: "#48bb78".to_string(),
            disabled_border: "#bcf0da".to_string(),
            selection_background: "#bbf7d0".to_string(),
            selection_text: "#14532d".to_string(),
        },
    }
} 