use crate::theme::{ComponentTheme, Mode, Season};
use crate::components::theme_selector::types::ThemeSelectorColors;

pub fn get_theme() -> ComponentTheme<ThemeSelectorColors> {
    ComponentTheme {
        name: "Theme Selector Light Summer".to_string(),
        mode: Mode::Light,
        season: Some(Season::Summer),
        colors: ThemeSelectorColors {
            button_bg: "#FFFBEB".to_string(),    // Very light amber
            button_text: "#111827".to_string(),   // Very dark gray
            button_border: "#F59E0B".to_string(), // Amber
            active_bg: "#D97706".to_string(),     // Darker amber
            active_text: "#FFFFFF".to_string(),   // White
            hover_bg: "#FDE68A".to_string(),      // Light amber
        }
    }
} 