use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::HelloColors;

pub fn get_theme() -> ComponentTheme<HelloColors> {
    ComponentTheme {
        name: "Dark Spring".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Spring),
        colors: HelloColors {
            background: "green-900".to_string(),
            surface: "green-800".to_string(),
            text: "white".to_string(),
            text_muted: "green-200".to_string(),
            border: "green-700".to_string(),
            border_hover: "green-500".to_string(),
            shadow_color: "green-900".to_string(),
            hover_background: "green-700".to_string(),
            hover_border: "green-500".to_string(),
            selection_background: "green-700".to_string(),
            selection_text: "white".to_string(),
            focus_border: "green-500".to_string(),
            focus_ring: "green-600".to_string(),
            active_background: "green-600".to_string(),
            active_border: "green-500".to_string(),
            heading_text: "green-100".to_string(),
        },
    }
} 