use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::ThemeSelectorColors;

pub fn get_theme() -> ComponentTheme<ThemeSelectorColors> {
    ComponentTheme {
        name: "Dark Spring".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Spring),
        colors: ThemeSelectorColors {
            button_bg: "#1c4532".to_string(),
            button_text: "#9ae6b4".to_string(),
            button_hover_bg: "#22543d".to_string(),
            button_border: "#48bb78".to_string(),
            select_bg: "#1c4532".to_string(),
            select_text: "#9ae6b4".to_string(),
            select_border: "#48bb78".to_string(),
            select_hover_bg: "#22543d".to_string(),
            icon_color: "#4fd1c5".to_string(),
        },
    }
} 