use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::NavbarColors;

pub fn get_theme() -> ComponentTheme<NavbarColors> {
    ComponentTheme {
        name: "Dark Summer".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Summer),
        colors: NavbarColors {
            background: "#742a2a".to_string(),
            text: "#feb2b2".to_string(),
            text_muted: "#f56565".to_string(),
            border: "#c53030".to_string(),
        },
    }
} 