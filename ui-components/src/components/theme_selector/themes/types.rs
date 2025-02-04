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
    fn apply_base_theme(&mut self, _base_theme: &BaseTheme) {
        // No-op: ThemeSelectorColors doesn't need to adjust based on base theme
        // Colors are fully defined by the theme implementations
    }
}

pub type ThemeSelectorTheme = ComponentTheme<ThemeSelectorColors>; 