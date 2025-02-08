use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::FootbarColors;

pub fn get_theme() -> ComponentTheme<FootbarColors> {
    ComponentTheme {
        name: "Dark Summer".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Summer),
        colors: FootbarColors {
            background: "#7B341E".to_string(),
            surface: "#7B341E".to_string(),
            text: "#FEEBC8".to_string(),
            text_muted: "#ED8936".to_string(),
            border: "#9C4221".to_string(),
        },
    }
} 