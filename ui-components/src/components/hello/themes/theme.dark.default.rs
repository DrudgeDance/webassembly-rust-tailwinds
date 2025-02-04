use crate::theme::{ComponentTheme, Mode};
use super::types::HelloColors;

pub fn get_theme() -> ComponentTheme<HelloColors> {
    ComponentTheme {
        name: "Dark Default".to_string(),
        mode: Mode::Dark,
        theme: None,
        colors: HelloColors {
            text: "#f7fafc".to_string(),
            text_muted: "#a0aec0".to_string(),
            background: "#2d3748".to_string(),
            border: "#4a5568".to_string(),
            shadow: "rgba(0, 0, 0, 0.25)".to_string(),
        },
    }
} 