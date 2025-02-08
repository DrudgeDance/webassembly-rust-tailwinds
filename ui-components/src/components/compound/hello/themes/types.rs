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
    pub focus_border: String,
    pub focus_ring: String,
    pub active_background: String,
    pub active_border: String,
    pub heading_text: String,
}

impl ComponentColors for HelloColors {
    fn from_theme(base_theme: &BaseTheme) -> Self {
        Self {
            background: base_theme.colors.background.clone(),
            surface: base_theme.colors.surface.clone(),
            text: base_theme.colors.text.clone(),
            text_muted: base_theme.colors.text_muted.clone(),
            border: base_theme.colors.primary.clone(),
            border_hover: base_theme.colors.primary.clone(),
            shadow_color: base_theme.colors.surface.clone(),
            hover_background: base_theme.colors.surface.clone(),
            hover_border: base_theme.colors.primary.clone(),
            selection_background: base_theme.colors.surface.clone(),
            selection_text: base_theme.colors.text.clone(),
            focus_border: base_theme.colors.primary.clone(),
            focus_ring: base_theme.colors.primary.clone(),
            active_background: base_theme.colors.surface.clone(),
            active_border: base_theme.colors.primary.clone(),
            heading_text: base_theme.colors.text.clone(),
        }
    }
}

pub type HelloTheme = ComponentTheme<HelloColors>; 