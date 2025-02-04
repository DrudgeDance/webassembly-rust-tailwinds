use crate::theme::{BaseTheme, ComponentColors, ComponentTheme};

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeSelectorColors {
    pub button_bg: String,
    pub button_text: String,
    pub button_hover_bg: String,
    pub button_border: String,
    pub select_bg: String,
    pub select_text: String,
    pub select_border: String,
    pub select_hover_bg: String,
    pub icon_color: String,
}

impl ComponentColors for ThemeSelectorColors {
    fn from_theme(base_theme: &BaseTheme) -> Self {
        Self {
            button_bg: base_theme.colors.surface.clone(),
            button_text: base_theme.colors.text.clone(),
            button_hover_bg: base_theme.colors.primary.clone(),
            button_border: base_theme.colors.secondary.clone(),
            select_bg: base_theme.colors.surface.clone(),
            select_text: base_theme.colors.text.clone(),
            select_border: base_theme.colors.secondary.clone(),
            select_hover_bg: base_theme.colors.primary.clone(),
            icon_color: base_theme.colors.text_muted.clone(),
        }
    }
}

pub type ThemeSelectorTheme = ComponentTheme<ThemeSelectorColors>; 