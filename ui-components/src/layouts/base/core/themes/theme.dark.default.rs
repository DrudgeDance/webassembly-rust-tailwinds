use crate::theme::{ComponentTheme, Mode};
use super::types::LayoutColors;

pub fn get_theme() -> ComponentTheme<LayoutColors> {
    ComponentTheme {
        name: "Dark Default".to_string(),
        mode: Mode::Dark,
        theme: None,
        colors: LayoutColors {
            background: "#1F2937".to_string(),
            surface: "#1F2937".to_string(),
            text: "#F9FAFB".to_string(),
            text_muted: "#9CA3AF".to_string(),
            border: "#374151".to_string(),
            shadow: "rgba(0, 0, 0, 0.25)".to_string(),
        },
    }
} 