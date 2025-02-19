use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::StandardColors;

pub fn get_theme() -> ComponentTheme<StandardColors> {
    ComponentTheme {
        name: "Dark Summer".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Summer),
        colors: StandardColors {
            background: "#7B341E".to_string(),
            surface: "#7B341E".to_string(),
            text: "#FEEBC8".to_string(),
            text_muted: "#ED8936".to_string(),
            primary: "#ED8936".to_string(),
        },
    }
} 