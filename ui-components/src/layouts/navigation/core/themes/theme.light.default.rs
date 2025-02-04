use crate::theme::{ComponentTheme, Mode};
use super::types::LayoutColors;

pub fn get_theme() -> ComponentTheme<LayoutColors> {
    ComponentTheme {
        name: "Light Default".to_string(),
        mode: Mode::Light,
        theme: None,
        colors: LayoutColors {
            background: "#ffffff".to_string(),
            surface: "#f8fafc".to_string(),
            text: "#1a202c".to_string(),
            text_muted: "#4a5568".to_string(),
            border: "#e2e8f0".to_string(),
            shadow: "rgba(0, 0, 0, 0.1)".to_string(),
        },
    }
} 