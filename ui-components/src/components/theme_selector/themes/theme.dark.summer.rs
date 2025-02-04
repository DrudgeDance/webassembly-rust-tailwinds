use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::ThemeSelectorColors;

pub fn get_theme() -> ComponentTheme<ThemeSelectorColors> {
    ComponentTheme {
        name: "Dark Summer".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Summer),
        colors: ThemeSelectorColors {
            button_bg: "#742a2a".to_string(),
            button_text: "#feb2b2".to_string(),
            button_hover_bg: "#9b2c2c".to_string(),
            button_border: "#f56565".to_string(),
            select_bg: "#742a2a".to_string(),
            select_text: "#feb2b2".to_string(),
            select_border: "#f56565".to_string(),
            select_hover_bg: "#9b2c2c".to_string(),
            icon_color: "#f56565".to_string(),
        },
    }
} 