use crate::theme::{ComponentTheme, Mode};
use super::types::ThemeSelectorColors;

pub fn get_theme() -> ComponentTheme<ThemeSelectorColors> {
    ComponentTheme {
        name: "Light Default".to_string(),
        mode: Mode::Light,
        theme: None,
        colors: ThemeSelectorColors {
            button_bg: "#ffffff".to_string(),
            button_text: "#000000".to_string(),
            button_hover_bg: "#f5f5f5".to_string(),
            button_border: "#e2e8f0".to_string(),
            select_bg: "#ffffff".to_string(),
            select_text: "#000000".to_string(),
            select_border: "#e2e8f0".to_string(),
            select_hover_bg: "#f5f5f5".to_string(),
            icon_color: "#666666".to_string(),
        },
    }
} 