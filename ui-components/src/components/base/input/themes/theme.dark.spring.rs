use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::{InputColors, InputTheme};

pub fn get_theme() -> InputTheme {
    ComponentTheme {
        name: "Dark Spring".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Spring),
        colors: InputColors {
            background: "#1c4532".to_string(),
            text: "#9ae6b4".to_string(),
            placeholder: "#48bb78".to_string(),
            border: "#2f855a".to_string(),
            focus_ring: "#48bb78".to_string(),
            hover_background: "#2f855a".to_string(),
            disabled_background: "#2f855a".to_string(),
            disabled_text: "#48bb78".to_string(),
        },
    }
} 