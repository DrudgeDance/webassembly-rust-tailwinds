use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::ThemeSelectorColors;

pub fn get_theme() -> ComponentTheme<ThemeSelectorColors> {
    ComponentTheme {
        name: "Light Spring".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Spring),
        colors: ThemeSelectorColors {
            button_bg: "#f0fff4".to_string(),
            button_text: "#22543d".to_string(),
            button_hover_bg: "#e6ffed".to_string(),
            button_border: "#9ae6b4".to_string(),
            select_bg: "#f0fff4".to_string(),
            select_text: "#22543d".to_string(),
            select_border: "#9ae6b4".to_string(),
            select_hover_bg: "#e6ffed".to_string(),
            icon_color: "#48bb78".to_string(),
        },
    }
} 