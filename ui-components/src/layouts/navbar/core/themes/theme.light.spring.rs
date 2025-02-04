use super::types::NavbarTheme;
use crate::theme::{Mode, Theme};

pub fn get_theme() -> NavbarTheme {
    NavbarTheme {
        name: "Light Spring".into(),
        mode: Mode::Light,
        theme: Some(Theme::Spring),
        colors: super::types::NavbarColors {
            background: "#F0FFF4".into(),
            text: "#276749".into(),
            border: "#48BB78".into(),
        },
    }
} 