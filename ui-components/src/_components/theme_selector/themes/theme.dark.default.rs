use crate::theme::{ComponentTheme, Mode};
use super::types::ThemeSelectorColors;

pub fn get_theme() -> ComponentTheme<ThemeSelectorColors> {
    ComponentTheme {
        name: "Dark Default".to_string(),
        mode: Mode::Dark,
        theme: None,
        colors: ThemeSelectorColors {
            button_bg: "#1a1a1a".to_string(),
            button_text: "#ffffff".to_string(),
            button_hover_bg: "#2a2a2a".to_string(),
            button_border: "#4a5568".to_string(),
            select_bg: "#1a1a1a".to_string(),
            select_text: "#ffffff".to_string(),
            select_border: "#4a5568".to_string(),
            select_hover_bg: "#2a2a2a".to_string(),
            icon_color: "#999999".to_string(),
        },
    }
} 