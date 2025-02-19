use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::MainContentColors;

pub fn get_theme() -> ComponentTheme<MainContentColors> {
    ComponentTheme {
        name: "Light Spring".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Spring),
        colors: MainContentColors {
            background: "#f0fff4".to_string(),
            text: "#22543d".to_string(),
            text_muted: "#48bb78".to_string(),
            border: "#9ae6b4".to_string(),
            shadow: "rgba(72, 187, 120, 0.1)".to_string(),
        },
    }
} 