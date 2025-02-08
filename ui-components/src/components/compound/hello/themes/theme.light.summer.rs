use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::HelloColors;

pub fn get_theme() -> ComponentTheme<HelloColors> {
    ComponentTheme {
        name: "Light Summer".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Summer),
        colors: HelloColors {
            background: "orange-50".to_string(),
            surface: "orange-100".to_string(),
            text: "orange-900".to_string(),
            text_muted: "orange-600".to_string(),
            border: "orange-500".to_string(),
            border_hover: "orange-600".to_string(),
            shadow_color: "orange-200".to_string(),
            hover_background: "orange-200".to_string(),
            hover_border: "orange-600".to_string(),
            selection_background: "orange-200".to_string(),
            selection_text: "orange-900".to_string(),
            focus_border: "orange-600".to_string(),
            focus_ring: "orange-500".to_string(),
            active_background: "orange-300".to_string(),
            active_border: "orange-600".to_string(),
            heading_text: "orange-800".to_string(),
        },
    }
} 