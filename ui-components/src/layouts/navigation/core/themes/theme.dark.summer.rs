use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::LayoutColors;

pub fn get_theme() -> ComponentTheme<LayoutColors> {
    ComponentTheme {
        name: "Dark Summer".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Summer),
        colors: LayoutColors {
            background: "#742a2a".to_string(),
            surface: "#9b2c2c".to_string(),
            text: "#feb2b2".to_string(),
            text_muted: "#f56565".to_string(),
            border: "#c53030".to_string(),
            shadow: "rgba(245, 101, 101, 0.25)".to_string(),
        },
    }
} 