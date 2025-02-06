use crate::theme::{ComponentTheme, Mode};
use super::types::{InputColors, InputTheme};

pub fn get_theme() -> InputTheme {
    ComponentTheme {
        name: "Dark Default".to_string(),
        mode: Mode::Dark,
        theme: None,
        colors: InputColors {
            background: "#1F2937".to_string(),
            text: "#F9FAFB".to_string(),
            placeholder: "#9CA3AF".to_string(),
            border: "#374151".to_string(),
            focus_ring: "#60A5FA".to_string(),
            hover_background: "#374151".to_string(),
            disabled_background: "#374151".to_string(),
            disabled_text: "#6B7280".to_string(),
        },
    }
} 