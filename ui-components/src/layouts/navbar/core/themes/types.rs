use crate::theme::{BaseTheme, ComponentColors, ComponentTheme};

#[derive(Debug, Clone, PartialEq)]
pub struct NavbarColors {
    pub background: String,
    pub text: String,
    pub text_muted: String,
    pub border: String,
}

impl ComponentColors for NavbarColors {
    fn from_theme(base_theme: &BaseTheme) -> Self {
        Self {
            background: base_theme.colors.surface.clone(),
            text: base_theme.colors.text.clone(),
            text_muted: base_theme.colors.text_muted.clone(),
            border: base_theme.colors.primary.clone(),
        }
    }
}

pub type NavbarTheme = ComponentTheme<NavbarColors>; 