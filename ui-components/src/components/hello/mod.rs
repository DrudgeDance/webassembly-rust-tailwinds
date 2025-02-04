mod component;
mod themes;
mod theme_switcher;

pub use component::Hello;

use crate::theme::ComponentTheme;

#[derive(Debug, Clone)]
pub struct HelloColors {
    pub text: String,
    pub text_muted: String,
    pub background: String,
}

impl Default for HelloColors {
    fn default() -> Self {
        Self {
            text: "#000000".to_string(),
            text_muted: "#666666".to_string(),
            background: "#ffffff".to_string(),
        }
    }
}

pub type HelloTheme = ComponentTheme<HelloColors>; 