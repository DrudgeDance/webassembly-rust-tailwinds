use crate::theme::{ComponentColors, BaseTheme};

#[derive(Debug, Clone, PartialEq)]
pub struct HelloColors {
    pub text: String,
    pub background: String,
    pub surface: String,
    pub shadow: String,
}

impl ComponentColors for HelloColors {
    fn apply_base_theme(&mut self, base_theme: &BaseTheme) {
        self.text = base_theme.colors.text.clone();
        self.background = base_theme.colors.background.clone();
        self.surface = base_theme.colors.surface.clone();
    }
} 