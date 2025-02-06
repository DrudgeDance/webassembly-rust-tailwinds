use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::LayoutColors;

pub fn get_theme() -> ComponentTheme<LayoutColors> {
    ComponentTheme {
        name: "Dark Spring".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Spring),
        colors: LayoutColors {
            background: "green-900".to_string(),
            surface: "green-800".to_string(),
            text: "green-100".to_string(),
            text_muted: "green-400".to_string(),
            border: "green-600".to_string(),
            shadow: "green-700".to_string(),
        },
    }
} 