use crate::theme::{ComponentTheme, Mode, Theme};
use crate::components::hello::types::HelloColors;

pub fn get_theme() -> ComponentTheme<HelloColors> {
    ComponentTheme {
        name: "Hello Dark Summer".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Summer),
        colors: HelloColors {
            text: "#F9FAFB".to_string(),         // Very light gray
            background: "#1F2937".to_string(),    // Dark cool gray
            surface: "#374151".to_string(),      // Cool gray
            shadow: "rgba(245, 158, 11, 0.2)".to_string(), // Dark amber shadow
        }
    }
} 