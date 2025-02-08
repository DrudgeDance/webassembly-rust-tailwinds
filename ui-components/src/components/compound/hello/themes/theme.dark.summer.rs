use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::HelloColors;

pub fn get_theme() -> ComponentTheme<HelloColors> {
    ComponentTheme {
        name: "Dark Summer".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Summer),
        colors: HelloColors {
            background: "orange-900".to_string(),
            surface: "orange-800".to_string(),
            text: "white".to_string(),
            text_muted: "orange-200".to_string(),
            border: "orange-600".to_string(),
            border_hover: "orange-500".to_string(),
            shadow_color: "orange-900".to_string(),
            hover_background: "orange-700".to_string(),
            hover_border: "orange-500".to_string(),
            selection_background: "orange-700".to_string(),
            selection_text: "white".to_string(),
        },
    }
} 