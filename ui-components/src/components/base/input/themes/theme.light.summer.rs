use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::{InputColors, InputTheme};

pub fn get_theme() -> InputTheme {
    ComponentTheme {
        name: "Light Summer".to_string(),
        mode: Mode::Light,
        theme: Some(Theme::Summer),
        colors: InputColors {
            background: "#fff5eb".to_string(),    // Warm white
            text: "#93370d".to_string(),          // Deep terracotta
            placeholder: "#f97316".to_string(),    // Bright orange
            border: "#fdba74".to_string(),        // Soft peach
            initial_focus_ring: "#f97316".to_string(), // Bright orange
            focus_ring: "#f97316".to_string(),    // Bright orange
            focus_border: "#f97316".to_string(),  // Bright orange
            hover_background: "#ffedd5".to_string(), // Light peach
            hover_border: "#fb923c".to_string(),   // Muted orange
            disabled_background: "#fff5eb".to_string(), // Warm white
            disabled_text: "#fb923c".to_string(),  // Muted orange
            disabled_border: "#fed7aa".to_string(),
            selection_background: "#fed7aa".to_string(),
            selection_text: "#7c2d12".to_string(),
        },
    }
} 