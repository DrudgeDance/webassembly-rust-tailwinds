use crate::theme::{BaseTheme, ComponentTheme, ComponentColors};

#[derive(Debug, Clone, PartialEq)]
pub struct InputColors {
    pub background: String,
    pub text: String,
    pub placeholder: String,
    pub border: String,
    pub initial_focus_ring: String,  // For immediate focus state
    pub focus_ring: String,
    pub focus_border: String,
    pub hover_background: String,
    pub hover_border: String,
    pub disabled_background: String,
    pub disabled_text: String,
    pub disabled_border: String,
    pub selection_background: String,
    pub selection_text: String,
}

impl ComponentColors for InputColors {
    fn from_theme(_base_theme: &BaseTheme) -> Self {
        Self {
            background: "white".to_string(),
            text: "gray-900".to_string(),
            placeholder: "gray-500".to_string(),
            border: "blue-500/50".to_string(),
            initial_focus_ring: "blue-500/50".to_string(),
            focus_ring: "blue-500/50".to_string(),
            focus_border: "blue-500".to_string(),
            hover_background: "gray-50".to_string(),
            hover_border: "blue-400".to_string(),
            disabled_background: "gray-50".to_string(),
            disabled_text: "gray-300".to_string(),
            disabled_border: "gray-200".to_string(),
            selection_background: "blue-100".to_string(),
            selection_text: "blue-900".to_string(),
        }
    }
}

pub type InputTheme = ComponentTheme<InputColors>; 