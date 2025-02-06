use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::{InputColors, InputTheme};

pub fn get_theme() -> InputTheme {
    ComponentTheme {
        name: "Light Summer".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Summer),
        colors: InputColors {
            background: "#fffaf0".to_string(),
            text: "#7b341e".to_string(),
            placeholder: "#ed8936".to_string(),
            border: "#fbd38d".to_string(),
            focus_ring: "#ed8936".to_string(),
            hover_background: "#fff5eb".to_string(),
            disabled_background: "#fff5eb".to_string(),
            disabled_text: "#ed8936".to_string(),
        },
    }
} 