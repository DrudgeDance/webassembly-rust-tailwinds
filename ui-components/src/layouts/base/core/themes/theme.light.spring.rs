use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::LayoutColors;

pub fn get_theme() -> ComponentTheme<LayoutColors> {
    ComponentTheme {
        name: "Light Spring".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Spring),
        colors: LayoutColors {
            background: "green-50".to_string(),
            surface: "white".to_string(),
            text: "green-900".to_string(),
            text_muted: "green-600".to_string(),
            border: "green-200".to_string(),
            shadow: "green-200".to_string(),
        },
    }
} 