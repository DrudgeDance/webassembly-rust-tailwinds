use crate::theme::{ComponentTheme, Mode, Theme};
use super::types::{InputColors, InputTheme};

pub fn get_theme() -> InputTheme {
    ComponentTheme {
        name: "Dark Spring".to_string(),
        mode: Mode::Dark,
        theme: Some(Theme::Spring),
        colors: InputColors {
            background: "#1a2e1f".to_string(),      // Dark forest green
            text: "#c1f0c1".to_string(),            // Light mint green
            placeholder: "#4ade80".to_string(),      // Bright spring green
            border: "#2d503f".to_string(),          // Deep forest green
            initial_focus_ring: "#4ade80".to_string(), // Bright spring green
            focus_ring: "#4ade80".to_string(),      // Bright spring green
            focus_border: "#4ade80".to_string(),    // Bright spring green
            hover_background: "#2d503f".to_string(), // Deep forest green
            hover_border: "#3d6657".to_string(),    // Lighter forest green
            disabled_background: "#243b2d".to_string(), // Muted forest green
            disabled_text: "#65a88a".to_string(),    // Muted sage green
            disabled_border: "#1f3228".to_string(),  // Very dark forest green
            selection_background: "#4ade80".to_string(), // Bright spring green
            selection_text: "#1a2e1f".to_string(),   // Dark forest green
        },
    }
} 