use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::MainContentColors;

pub fn get_theme() -> ComponentTheme<MainContentColors> {
    ComponentTheme {
        name: "Dark Summer".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Summer),
        colors: MainContentColors {
            background: "#7b341e".to_string(),
            text: "#fbd38d".to_string(),
            text_muted: "#ed8936".to_string(),
            border: "#c05621".to_string(),
            shadow: "rgba(237, 137, 54, 0.25)".to_string(),
        },
    }
} 