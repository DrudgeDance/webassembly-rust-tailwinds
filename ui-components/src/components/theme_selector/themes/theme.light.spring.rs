use crate::theme::{ComponentTheme, Mode, Theme};
use crate::components::theme_selector::types::ThemeSelectorColors;

pub fn get_theme() -> ComponentTheme<ThemeSelectorColors> {
    ComponentTheme {
        name: "Theme Selector Light Spring".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Spring),
        colors: ThemeSelectorColors {
            button_bg: "#F0FFF4".to_string(),    // Very light spring green
            button_text: "#111827".to_string(),   // Very dark gray
            button_border: "#10B981".to_string(), // Emerald
            active_bg: "#059669".to_string(),     // Emerald darker
            active_text: "#FFFFFF".to_string(),   // White
            hover_bg: "#D1FAE5".to_string(),      // Light emerald
        }
    }
} 