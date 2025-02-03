use crate::theme::{ComponentColors, Theme};

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeSelectorColors {
    pub button_bg: String,
    pub button_text: String,
    pub button_border: String,
    pub active_bg: String,
    pub active_text: String,
    pub hover_bg: String,
}

impl ComponentColors for ThemeSelectorColors {
    fn apply_base_theme(&mut self, base_theme: &Theme) {
        self.button_bg = base_theme.colors.surface.clone();
        self.button_text = base_theme.colors.text.clone();
        self.button_border = base_theme.colors.surface.clone();
        self.active_bg = base_theme.colors.primary.clone();
        self.active_text = base_theme.colors.text.clone();
        self.hover_bg = base_theme.colors.surface.clone();
    }
} 