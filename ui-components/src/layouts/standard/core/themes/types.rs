use crate::theme::{BaseTheme, ComponentTheme, ComponentColors};

#[derive(Debug, Clone, PartialEq)]
pub struct StandardColors {
    pub background: String,
    pub surface: String,
    pub text: String,
    pub text_muted: String,
    pub border: String,
    pub shadow: String,
    pub primary: String,
    pub primary_hover: String,
    pub error: String,
}

impl ComponentColors for StandardColors {
    fn from_theme(base_theme: &BaseTheme) -> Self {
        Self {
            background: base_theme.colors.background.clone(),
            surface: base_theme.colors.background.clone(), // Using same as background for consistency
            text: base_theme.colors.text.clone(),
            text_muted: base_theme.colors.text_muted.clone(),
            border: base_theme.colors.primary.clone(),
            shadow: "rgba(0, 0, 0, 0.1)".to_string(),
            primary: base_theme.colors.primary.clone(),
            primary_hover: base_theme.colors.secondary.clone(),
            error: "#ef4444".to_string(),
        }
    }
}

pub type StandardTheme = ComponentTheme<StandardColors>; 