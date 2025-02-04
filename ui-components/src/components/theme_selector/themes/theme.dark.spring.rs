use crate::theme::{ComponentTheme, Mode, Theme};
use crate::components::theme_selector::types::ThemeSelectorColors;

pub fn get_theme() -> ComponentTheme<ThemeSelectorColors> {
    ComponentTheme {
        name: "Theme Selector Dark Spring".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Spring),
        colors: ThemeSelectorColors {
            button_bg: "#1A2F1A".to_string(),    // Dark spring green
            button_text: "#F9FAFB".to_string(),   // Very light gray
            button_border: "#10B981".to_string(), // Emerald
            active_bg: "#059669".to_string(),     // Emerald darker
            active_text: "#FFFFFF".to_string(),   // White
            hover_bg: "#2D3D2D".to_string(),      // Lighter spring green
        }
    }
} 