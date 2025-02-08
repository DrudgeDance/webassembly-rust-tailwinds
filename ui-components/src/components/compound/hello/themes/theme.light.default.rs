use crate::theme::{ComponentTheme, Mode};
use super::types::HelloColors;

pub fn get_theme() -> ComponentTheme<HelloColors> {
    ComponentTheme {
        name: "Light Default".to_string(),
        mode: Mode::Light,
        theme: None,
        colors: HelloColors {
            background: "blue-50".to_string(),
            surface: "blue-100".to_string(),
            text: "blue-900".to_string(),
            text_muted: "blue-600".to_string(),
            border: "blue-400".to_string(),
            border_hover: "blue-500".to_string(),
            shadow_color: "blue-200".to_string(),
            hover_background: "blue-200".to_string(),
            hover_border: "blue-500".to_string(),
            selection_background: "blue-200".to_string(),
            selection_text: "blue-900".to_string(),
            focus_border: "blue-500".to_string(),
            focus_ring: "blue-400".to_string(),
            active_background: "blue-300".to_string(),
            active_border: "blue-500".to_string(),
            heading_text: "blue-800".to_string(),
        },
    }
} 