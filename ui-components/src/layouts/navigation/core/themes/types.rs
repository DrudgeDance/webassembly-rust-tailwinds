use crate::theme::{BaseTheme, ComponentColors, ComponentTheme};

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutColors {
    pub background: String,
    pub surface: String,
    pub text: String,
    pub text_muted: String,
    pub border: String,
    pub shadow: String,
}

impl ComponentColors for LayoutColors {
    fn from_theme(base_theme: &BaseTheme) -> Self {
        Self {
            background: base_theme.colors.background.clone(),
            surface: base_theme.colors.surface.clone(),
            text: base_theme.colors.text.clone(),
            text_muted: base_theme.colors.text_muted.clone(),
            border: base_theme.colors.primary.clone(),
            shadow: base_theme.colors.secondary.clone(),
        }
    }
}

pub type LayoutTheme = ComponentTheme<LayoutColors>; 