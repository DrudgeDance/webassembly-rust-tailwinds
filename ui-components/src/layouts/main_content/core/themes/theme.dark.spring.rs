use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::MainContentColors;

pub fn get_theme() -> ComponentTheme<MainContentColors> {
    ComponentTheme {
        name: "Dark Spring".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Spring),
        colors: MainContentColors {
            background: "#22543d".to_string(),
            text: "#9ae6b4".to_string(),
            text_muted: "#48bb78".to_string(),
            border: "#2f855a".to_string(),
            shadow: "rgba(72, 187, 120, 0.25)".to_string(),
        },
    }
} 