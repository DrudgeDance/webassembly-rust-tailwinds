use crate::theme::{ComponentTheme, Mode};
use super::types::FootbarColors;

pub fn get_theme() -> ComponentTheme<FootbarColors> {
    ComponentTheme {
        name: "Dark Default".to_string(),
        mode: Mode::Dark,
        theme: None,
        colors: FootbarColors {
            background: "#1F2937".to_string(),
            surface: "#1F2937".to_string(),
            text: "#F9FAFB".to_string(),
            text_muted: "#9CA3AF".to_string(),
            border: "#374151".to_string(),
        },
    }
} 