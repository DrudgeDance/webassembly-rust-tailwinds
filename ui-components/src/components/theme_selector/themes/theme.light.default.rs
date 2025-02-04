use crate::theme::{ComponentTheme, Mode};
use crate::components::theme_selector::types::ThemeSelectorColors;

pub fn get_theme() -> ComponentTheme<ThemeSelectorColors> {
    ComponentTheme {
        name: "Theme Selector Light Default".to_string(),
        mode: Mode::Light,
        theme: None,
        colors: ThemeSelectorColors {
            button_bg: "#F3F4F6".to_string(),    // Very light gray
            button_text: "#111827".to_string(),   // Very dark gray
            button_border: "#D1D5DB".to_string(), // Light gray
            active_bg: "#4F46E5".to_string(),     // Indigo
            active_text: "#FFFFFF".to_string(),   // White
            hover_bg: "#E5E7EB".to_string(),      // Light gray hover
        }
    }
} 