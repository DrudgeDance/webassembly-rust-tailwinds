use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::StandardColors;

pub fn get_theme() -> ComponentTheme<StandardColors> {
    ComponentTheme {
        name: "Light Summer".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Summer),
        colors: StandardColors {
            background: "#fffaf0".to_string(),
            surface: "#fffaf0".to_string(),
            text: "#7b341e".to_string(),
            text_muted: "#ed8936".to_string(),
            primary: "#ed8936".to_string(),
        },
    }
} 