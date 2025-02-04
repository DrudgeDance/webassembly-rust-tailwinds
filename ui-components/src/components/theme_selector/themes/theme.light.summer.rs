use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::ThemeSelectorColors;

pub fn get_theme() -> ComponentTheme<ThemeSelectorColors> {
    ComponentTheme {
        name: "Light Summer".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Summer),
        colors: ThemeSelectorColors {
            button_bg: "#fff5f5".to_string(),
            button_text: "#c53030".to_string(),
            button_hover_bg: "#fed7d7".to_string(),
            button_border: "#fc8181".to_string(),
            select_bg: "#fff5f5".to_string(),
            select_text: "#c53030".to_string(),
            select_border: "#fc8181".to_string(),
            select_hover_bg: "#fed7d7".to_string(),
            icon_color: "#f56565".to_string(),
        },
    }
} 