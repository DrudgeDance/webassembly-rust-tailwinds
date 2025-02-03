use crate::theme::{ComponentTheme, Mode, Season};
use crate::components::theme_selector::types::ThemeSelectorColors;

pub fn get_theme() -> ComponentTheme<ThemeSelectorColors> {
    ComponentTheme {
        name: "Theme Selector Dark Summer".to_string(),
        mode: Mode::Dark,
        season: Some(Season::Summer),
        colors: ThemeSelectorColors {
            button_bg: "#78350F".to_string(),    // Dark amber
            button_text: "#F9FAFB".to_string(),   // Very light gray
            button_border: "#B45309".to_string(), // Medium amber
            active_bg: "#F59E0B".to_string(),     // Light amber
            active_text: "#FFFFFF".to_string(),   // White
            hover_bg: "#92400E".to_string(),      // Darker amber hover
        }
    }
} 