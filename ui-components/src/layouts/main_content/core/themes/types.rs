use crate::theme::{BaseTheme, ComponentColors, ComponentTheme};

#[derive(Debug, Clone, PartialEq)]
pub struct MainContentColors {
    pub background: String,
    pub text: String,
    pub text_muted: String,
    pub border: String,
    pub shadow: String,
}

impl ComponentColors for MainContentColors {
    fn from_theme(base_theme: &BaseTheme) -> Self {
        Self {
            background: base_theme.colors.background.clone(),
            text: base_theme.colors.text.clone(),
            text_muted: base_theme.colors.text_muted.clone(),
            border: base_theme.colors.primary.clone(),
            shadow: base_theme.colors.secondary.clone(),
        }
    }
}

pub type MainContentTheme = ComponentTheme<MainContentColors>; 