use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::LayoutColors;

pub fn get_theme() -> ComponentTheme<LayoutColors> {
    ComponentTheme {
        name: "Light Summer".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Summer),
        colors: LayoutColors {
            background: "#fff5f5".to_string(),
            surface: "#fff5f5".to_string(),
            text: "#c53030".to_string(),
            text_muted: "#f56565".to_string(),
            border: "#fc8181".to_string(),
            shadow: "rgba(245, 101, 101, 0.1)".to_string(),
        },
    }
} 