use super::types::NavbarTheme;
use crate::theme::Mode;

pub fn get_theme() -> NavbarTheme {
    NavbarTheme {
        name: "Dark Default".into(),
        mode: Mode::Dark,
        theme: None,
        colors: super::types::NavbarColors {
            background: "#1F2937".into(),
            text: "#FFFFFF".into(),
            border: "#3B82F6".into(),
        },
    }
} 