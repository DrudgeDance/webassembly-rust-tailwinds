use crate::theme::{ComponentTheme, Mode};
use super::types::FootbarColors;

pub fn get_theme() -> ComponentTheme<FootbarColors> {
    ComponentTheme {
        name: "Light Default".to_string(),
        mode: Mode::Light,
        theme: None,
        colors: FootbarColors {
            background: "#ffffff".to_string(),
            surface: "#ffffff".to_string(),
            text: "#1a202c".to_string(),
            text_muted: "#4a5568".to_string(),
            border: "#e2e8f0".to_string(),
        },
    }
} 