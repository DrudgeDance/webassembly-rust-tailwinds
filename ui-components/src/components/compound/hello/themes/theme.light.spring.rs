use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::HelloColors;

pub fn get_theme() -> ComponentTheme<HelloColors> {
    ComponentTheme {
        name: "Light Spring".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Spring),
        colors: HelloColors {
            background: "green-50".to_string(),
            surface: "green-100".to_string(),
            text: "green-900".to_string(),
            text_muted: "green-600".to_string(),
            border: "green-500".to_string(),
            border_hover: "green-600".to_string(),
            shadow_color: "green-200".to_string(),
            hover_background: "green-200".to_string(),
            hover_border: "green-600".to_string(),
            selection_background: "green-200".to_string(),
            selection_text: "green-900".to_string(),
            focus_border: "green-600".to_string(),
            focus_ring: "green-500".to_string(),
            active_background: "green-300".to_string(),
            active_border: "green-600".to_string(),
            heading_text: "green-800".to_string(),
        },
    }
} 