use crate::theme::{ComponentTheme, Mode};
use super::types::StandardColors;

pub fn get_theme() -> ComponentTheme<StandardColors> {
    ComponentTheme {
        name: "Dark Default".to_string(),
        mode: Mode::Dark,
        theme: None,
        colors: StandardColors {
            background: "#1F2937".to_string(),
            surface: "#1F2937".to_string(),
            text: "#F9FAFB".to_string(),
            text_muted: "#9CA3AF".to_string(),
            border: "#374151".to_string(),
            shadow: "rgba(0, 0, 0, 0.25)".to_string(),
            primary: "#60A5FA".to_string(),
            primary_hover: "#3B82F6".to_string(),
            error: "#F87171".to_string(),
        },
    }
} 