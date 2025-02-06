use crate::theme::{ComponentTheme, Mode};
use super::types::{InputColors, InputTheme};

pub fn get_theme() -> InputTheme {
    ComponentTheme {
        name: "Light Default".to_string(),
        mode: Mode::Light,
        theme: None,
        colors: InputColors {
            background: "#ffffff".to_string(),
            text: "#1a202c".to_string(),
            placeholder: "#4a5568".to_string(),
            border: "#e2e8f0".to_string(),
            initial_focus_ring: "#3b82f6".to_string(),
            focus_ring: "#3b82f6".to_string(),
            focus_border: "#3b82f6".to_string(),
            hover_background: "#f8fafc".to_string(),
            hover_border: "#cbd5e1".to_string(),
            disabled_background: "#f1f5f9".to_string(),
            disabled_text: "#94a3b8".to_string(),
            disabled_border: "#e2e8f0".to_string(),
            selection_background: "#bfdbfe".to_string(),
            selection_text: "#1e3a8a".to_string(),
        },
    }
} 