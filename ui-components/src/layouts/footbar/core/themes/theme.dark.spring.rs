use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::FootbarColors;

pub fn get_theme() -> ComponentTheme<FootbarColors> {
    ComponentTheme {
        name: "Dark Spring".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Spring),
        colors: FootbarColors {
            background: "#1c4532".to_string(),
            surface: "#1c4532".to_string(),
            text: "#9ae6b4".to_string(),
            text_muted: "#48bb78".to_string(),
            border: "#2f855a".to_string(),
        },
    }
} 