use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::LayoutColors;

pub fn get_theme() -> ComponentTheme<LayoutColors> {
    ComponentTheme {
        name: "Light Summer".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Summer),
        colors: LayoutColors {
            background: "orange-50".to_string(),
            surface: "white".to_string(),
            text: "orange-900".to_string(),
            text_muted: "orange-600".to_string(),
            border: "orange-200".to_string(),
            shadow: "orange-200".to_string(),
        },
    }
} 