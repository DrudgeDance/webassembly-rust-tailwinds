use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::{InputColors, InputTheme};

pub fn get_theme() -> InputTheme {
    ComponentTheme {
        name: "Dark Summer".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Summer),
        colors: InputColors {
            background: "#7B341E".to_string(),
            text: "#FEEBC8".to_string(),
            placeholder: "#ED8936".to_string(),
            border: "#9C4221".to_string(),
            focus_ring: "#ED8936".to_string(),
            hover_background: "#9C4221".to_string(),
            disabled_background: "#9C4221".to_string(),
            disabled_text: "#ED8936".to_string(),
        },
    }
} 