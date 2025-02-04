use super::types::NavbarTheme;
use crate::theme::{Mode, Theme};

pub fn get_theme() -> NavbarTheme {
    NavbarTheme {
        name: "Light Summer".into(),
        mode: Mode::Light,
        theme: Some(Theme::Summer),
        colors: super::types::NavbarColors {
            background: "#FFFFF0".into(),
            text: "#C05621".into(),
            border: "#ED8936".into(),
        },
    }
} 