use super::types::NavbarTheme;
use crate::theme::{Mode, Theme};

pub fn get_theme() -> NavbarTheme {
    NavbarTheme {
        name: "Dark Spring".into(),
        mode: Mode::Dark,
        theme: Some(Theme::Spring),
        colors: super::types::NavbarColors {
            background: "#1C4532".into(),
            text: "#9AE6B4".into(),
            border: "#48BB78".into(),
        },
    }
} 