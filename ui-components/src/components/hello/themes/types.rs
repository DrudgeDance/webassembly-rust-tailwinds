use crate::theme::{BaseTheme, ComponentTheme, ComponentColors};

#[derive(Debug, Clone, PartialEq)]
pub struct HelloColors {
    pub background: String,
    pub surface: String,
    pub text: String,
    pub text_muted: String,
    pub border: String,
    pub border_hover: String,
    pub shadow_color: String,
    pub hover_background: String,
    pub hover_border: String,
    pub selection_background: String,
    pub selection_text: String,
}

impl ComponentColors for HelloColors {
    fn from_theme(_base_theme: &BaseTheme) -> Self {
        Self {
            background: "white".to_string(),
            surface: "gray-50".to_string(),
            text: "gray-900".to_string(),
            text_muted: "gray-600".to_string(),
            border: "gray-200".to_string(),
            border_hover: "gray-300".to_string(),
            shadow_color: "gray-200".to_string(),
            hover_background: "gray-50".to_string(),
            hover_border: "gray-300".to_string(),
            selection_background: "gray-100".to_string(),
            selection_text: "gray-900".to_string(),
        }
    }
}

pub type HelloTheme = ComponentTheme<HelloColors>; 