use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::LayoutColors;

pub fn get_theme() -> ComponentTheme<LayoutColors> {
    ComponentTheme {
        name: "Dark Summer".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Summer),
        colors: LayoutColors {
            background: "#7B341E".to_string(),
            surface: "#7B341E".to_string(),
            text: "#FEEBC8".to_string(),
            text_muted: "#ED8936".to_string(),
            border: "#9C4221".to_string(),
            shadow: "rgba(237, 137, 54, 0.25)".to_string(),
        },
    }
} 