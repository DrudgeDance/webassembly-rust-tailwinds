use crate::theme::{ComponentTheme, Mode};
use super::types::StandardColors;

pub fn get_theme() -> ComponentTheme<StandardColors> {
    ComponentTheme {
        name: "Light Default".to_string(),
        mode: Mode::Light,
        theme: None,
        colors: StandardColors {
            background: "#ffffff".to_string(),
            surface: "#ffffff".to_string(),
            text: "#1a202c".to_string(),
            text_muted: "#4a5568".to_string(),
            border: "#e2e8f0".to_string(),
            shadow: "rgba(0, 0, 0, 0.1)".to_string(),
            primary: "#3b82f6".to_string(),
            primary_hover: "#2563eb".to_string(),
            error: "#ef4444".to_string(),
        },
    }
} 