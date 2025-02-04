use crate::theme::{ComponentTheme, Mode, Theme};
use crate::components::hello::types::HelloColors;

pub fn get_theme() -> ComponentTheme<HelloColors> {
    ComponentTheme {
        name: "Hello Light Spring".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Spring),
        colors: HelloColors {
            text: "#111827".to_string(),         // Very dark gray
            background: "#F0FFF4".to_string(),    // Very light spring green
            surface: "#FFFFFF".to_string(),       // White
            shadow: "rgba(16, 185, 129, 0.1)".to_string(), // Light emerald shadow
        }
    }
} 