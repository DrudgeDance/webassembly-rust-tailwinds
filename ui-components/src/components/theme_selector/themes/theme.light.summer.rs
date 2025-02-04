use crate::theme::{ComponentTheme, Mode, Theme};
use crate::components::theme_selector::types::ThemeSelectorColors;

pub fn get_theme() -> ComponentTheme<ThemeSelectorColors> {
    ComponentTheme {
        name: "Theme Selector Light Summer".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Summer),
        colors: ThemeSelectorColors {
            button_bg: "#FFFAF0".to_string(),    // Floral white
            button_text: "#111827".to_string(),   // Very dark gray
            button_border: "#F59E0B".to_string(), // Amber
            active_bg: "#D97706".to_string(),     // Amber darker
            active_text: "#FFFFFF".to_string(),   // White
            hover_bg: "#FEF3C7".to_string(),      // Light amber
        }
    }
} 