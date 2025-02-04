use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::HelloColors;

pub fn get_theme() -> ComponentTheme<HelloColors> {
    ComponentTheme {
        name: "Dark Spring".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Spring),
        colors: HelloColors {
            text: "#9ae6b4".to_string(),
            text_muted: "#48bb78".to_string(),
            background: "#22543d".to_string(),
            border: "#2f855a".to_string(),
            shadow: "rgba(72, 187, 120, 0.25)".to_string(),
        },
    }
} 