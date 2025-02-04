use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::StandardColors;

pub fn get_theme() -> ComponentTheme<StandardColors> {
    ComponentTheme {
        name: "Light Spring".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Spring),
        colors: StandardColors {
            background: "#f0fff4".to_string(),
            surface: "#f0fff4".to_string(),
            text: "#22543d".to_string(),
            text_muted: "#48bb78".to_string(),
            border: "#9ae6b4".to_string(),
            shadow: "rgba(72, 187, 120, 0.1)".to_string(),
            primary: "#48bb78".to_string(),
            primary_hover: "#38a169".to_string(),
            error: "#f87171".to_string(),
        },
    }
} 