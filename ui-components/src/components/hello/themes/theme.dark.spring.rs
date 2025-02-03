use crate::theme::{ComponentTheme, Mode, Season};
use crate::components::hello::types::HelloColors;

pub fn get_theme() -> ComponentTheme<HelloColors> {
    ComponentTheme {
        name: "Hello Dark Spring".to_string(),
        mode: Mode::Dark,
        season: Some(Season::Spring),
        colors: HelloColors {
            text: "#F9FAFB".to_string(),         // Very light gray
            background: "#1A2F1A".to_string(),    // Dark spring green
            surface: "#2D3D2D".to_string(),      // Lighter spring green
            shadow: "rgba(16, 185, 129, 0.2)".to_string(), // Dark emerald shadow
        }
    }
} 