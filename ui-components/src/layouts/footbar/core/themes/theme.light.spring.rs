use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::FootbarColors;

pub fn get_theme() -> ComponentTheme<FootbarColors> {
    ComponentTheme {
        name: "Light Spring".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Spring),
        colors: FootbarColors {
            background: "#f0fff4".to_string(),
            surface: "#f0fff4".to_string(),
            text: "#22543d".to_string(),
            text_muted: "#48bb78".to_string(),
            border: "#9ae6b4".to_string(),
        },
    }
} 