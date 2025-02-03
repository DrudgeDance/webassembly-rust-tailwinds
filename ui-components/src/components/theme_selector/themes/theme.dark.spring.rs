use crate::theme::{ComponentTheme, Mode, Season};
use crate::components::theme_selector::types::ThemeSelectorColors;

pub fn get_theme() -> ComponentTheme<ThemeSelectorColors> {
    ComponentTheme {
        name: "Theme Selector Dark Spring".to_string(),
        mode: Mode::Dark,
        season: Some(Season::Spring),
        colors: ThemeSelectorColors {
            button_bg: "#064E3B".to_string(),    // Dark emerald
            button_text: "#F9FAFB".to_string(),   // Very light gray
            button_border: "#059669".to_string(), // Emerald
            active_bg: "#10B981".to_string(),     // Light emerald
            active_text: "#FFFFFF".to_string(),   // White
            hover_bg: "#065F46".to_string(),      // Darker emerald hover
        }
    }
} 