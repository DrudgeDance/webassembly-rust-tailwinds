use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::LayoutColors;

pub fn get_theme() -> ComponentTheme<LayoutColors> {
    ComponentTheme {
        name: "Dark Summer".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Summer),
        colors: LayoutColors {
            background: "orange-900".to_string(),
            surface: "orange-800".to_string(),
            text: "orange-100".to_string(),
            text_muted: "orange-400".to_string(),
            border: "orange-600".to_string(),
            shadow: "orange-700".to_string(),
        },
    }
}