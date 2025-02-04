use crate::theme::{ComponentTheme, Mode, Theme};
use crate::components::hello::types::HelloColors;

pub fn get_theme() -> ComponentTheme<HelloColors> {
    ComponentTheme {
        name: "Hello Light Summer".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Summer),
        colors: HelloColors {
            text: "#111827".to_string(),         // Very dark gray
            background: "#FFFAF0".to_string(),    // Floral white
            surface: "#FFFFFF".to_string(),       // White
            shadow: "rgba(245, 158, 11, 0.1)".to_string(), // Light amber shadow
        }
    }
} 