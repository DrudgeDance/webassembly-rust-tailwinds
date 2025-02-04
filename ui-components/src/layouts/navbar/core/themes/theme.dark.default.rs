use crate::theme::{ComponentTheme, Mode};
use super::types::NavbarColors;

pub fn get_theme() -> ComponentTheme<NavbarColors> {
    ComponentTheme {
        name: "Dark Default".to_string(),
        mode: Mode::Dark,
        theme: None,
        colors: NavbarColors {
            background: "#1a202c".to_string(),
            text: "#f7fafc".to_string(),
            text_muted: "#a0aec0".to_string(),
            border: "#4a5568".to_string(),
        },
    }
} 