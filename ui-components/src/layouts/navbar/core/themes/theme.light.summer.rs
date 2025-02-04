use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::NavbarColors;

pub fn get_theme() -> ComponentTheme<NavbarColors> {
    ComponentTheme {
        name: "Light Summer".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Summer),
        colors: NavbarColors {
            background: "#fff5f5".to_string(),
            text: "#c53030".to_string(),
            text_muted: "#f56565".to_string(),
            border: "#fc8181".to_string(),
        },
    }
} 