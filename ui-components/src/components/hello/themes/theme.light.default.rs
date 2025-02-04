use crate::theme::{ComponentTheme, Mode};
use crate::components::hello::types::HelloColors;

pub fn get_theme() -> ComponentTheme<HelloColors> {
    ComponentTheme {
        name: "Hello Light Default".to_string(),
        mode: Mode::Light,
        theme: None,
        colors: HelloColors {
            text: "#111827".to_string(),         // Very dark gray
            background: "#F8FAF8".to_string(),    // Very light background
            surface: "#FFFFFF".to_string(),       // White
            shadow: "rgba(0, 0, 0, 0.1)".to_string(), // Light shadow
        }
    }
} 