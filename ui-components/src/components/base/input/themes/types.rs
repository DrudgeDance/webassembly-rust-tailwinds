use crate::theme::{BaseTheme, ComponentTheme, ComponentColors};

#[derive(Debug, Clone, PartialEq)]
pub struct InputColors {
    pub background: String,
    pub text: String,
    pub placeholder: String,
    pub border: String,
    pub focus_ring: String,
    pub hover_background: String,
    pub disabled_background: String,
    pub disabled_text: String,
}

impl ComponentColors for InputColors {
    fn from_theme(base_theme: &BaseTheme) -> Self {
        Self {
            background: base_theme.colors.background.clone(),
            text: base_theme.colors.text.clone(),
            placeholder: base_theme.colors.text_muted.clone(),
            border: base_theme.colors.primary.clone(),
            focus_ring: base_theme.colors.primary.clone(),
            hover_background: base_theme.colors.surface.clone(),
            disabled_background: base_theme.colors.surface.clone(),
            disabled_text: base_theme.colors.text_muted.clone(),
        }
    }
}

pub type InputTheme = ComponentTheme<InputColors>; 