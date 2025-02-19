use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::StandardColors;

pub fn get_theme() -> ComponentTheme<StandardColors> {
    ComponentTheme {
        name: "Dark Spring".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Spring),
        colors: StandardColors {
            background: "#1c4532".to_string(),
            surface: "#1c4532".to_string(),
            text: "#9ae6b4".to_string(),
            text_muted: "#48bb78".to_string(),
            primary: "#48bb78".to_string(),
        },
    }
} 