use super::types::NavbarTheme;
use crate::theme::Mode;

pub fn get_theme() -> NavbarTheme {
    NavbarTheme {
        name: "Light Default".into(),
        mode: Mode::Light,
        theme: None,
        colors: super::types::NavbarColors {
            background: "#FFFFFF".into(),
            text: "#000000".into(),
            border: "#3B82F6".into(),
        },
    }
} 