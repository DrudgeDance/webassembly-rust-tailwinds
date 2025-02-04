use crate::theme::{ComponentTheme, Mode};
use crate::components::theme_selector::types::ThemeSelectorColors;

pub fn get_theme() -> ComponentTheme<ThemeSelectorColors> {
    ComponentTheme {
        name: "Theme Selector Dark Default".to_string(),
        mode: Mode::Dark,
        theme: None,
        colors: ThemeSelectorColors {
            button_bg: "#374151".to_string(),    // Dark gray
            button_text: "#F9FAFB".to_string(),   // Very light gray
            button_border: "#4B5563".to_string(), // Medium gray
            active_bg: "#4F46E5".to_string(),     // Indigo
            active_text: "#FFFFFF".to_string(),   // White
            hover_bg: "#4B5563".to_string(),      // Medium gray hover
        }
    }
} 