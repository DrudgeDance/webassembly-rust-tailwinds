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
            primary: "#3b82f6".to_string(),
        },
    }
} 