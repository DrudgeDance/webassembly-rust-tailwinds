use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::LayoutColors;

pub fn get_theme() -> ComponentTheme<LayoutColors> {
    ComponentTheme {
        name: "Dark Spring".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Spring),
        colors: LayoutColors {
            background: "#1c4532".to_string(),
            surface: "#22543d".to_string(),
            text: "#9ae6b4".to_string(),
            text_muted: "#48bb78".to_string(),
            border: "#2f855a".to_string(),
            shadow: "rgba(72, 187, 120, 0.25)".to_string(),
        },
    }
} 