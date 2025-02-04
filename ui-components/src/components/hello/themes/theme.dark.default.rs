use crate::theme::{ComponentTheme, Mode};
use crate::components::hello::types::HelloColors;

pub fn get_theme() -> ComponentTheme<HelloColors> {
    ComponentTheme {
        name: "Hello Dark Default".to_string(),
        mode: Mode::Dark,
        theme: None,
        colors: HelloColors {
            text: "#F9FAFB".to_string(),         // Very light gray
            background: "#1A1C2A".to_string(),    // Dark blue-gray
            surface: "#2D2D3D".to_string(),      // Lighter blue-gray
            shadow: "rgba(0, 0, 0, 0.3)".to_string(), // Darker shadow
        }
    }
} 