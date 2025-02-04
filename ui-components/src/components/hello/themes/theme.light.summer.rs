use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::HelloColors;

pub fn get_theme() -> ComponentTheme<HelloColors> {
    ComponentTheme {
        name: "Light Summer".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Summer),
        colors: HelloColors {
            text: "#c53030".to_string(),
            text_muted: "#f56565".to_string(),
            background: "#fff5f5".to_string(),
            border: "#fc8181".to_string(),
            shadow: "rgba(245, 101, 101, 0.1)".to_string(),
        },
    }
} 