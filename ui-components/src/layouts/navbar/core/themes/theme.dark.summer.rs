use super::types::NavbarTheme;
use crate::theme::{Mode, Theme};

pub fn get_theme() -> NavbarTheme {
    NavbarTheme {
        name: "Dark Summer".into(),
        mode: Mode::Dark,
        theme: Some(Theme::Summer),
        colors: super::types::NavbarColors {
            background: "#7B341E".into(),
            text: "#FEEBC8".into(),
            border: "#ED8936".into(),
        },
    }
} 