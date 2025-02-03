mod types;
pub use types::*;

pub fn get_default_theme(mode: Mode) -> Theme {
    match mode {
        Mode::Light => Theme {
            name: "Light Default".to_string(),
            mode: Mode::Light,
            season: None,
            colors: ThemeColors {
                background: "#F8FAF8".to_string(),
                surface: "#FFFFFF".to_string(),
                text: "#111827".to_string(),
                text_muted: "#6B7280".to_string(),
                primary: "#4F46E5".to_string(),
                secondary: "#7C3AED".to_string(),
                accent: "#EC4899".to_string(),
                seasonal_primary: "#4F46E5".to_string(),
                seasonal_secondary: "#7C3AED".to_string(),
                seasonal_accent: "#EC4899".to_string(),
            }
        },
        Mode::Dark => Theme {
            name: "Dark Default".to_string(),
            mode: Mode::Dark,
            season: None,
            colors: ThemeColors {
                background: "#1A1C2A".to_string(),
                surface: "#2D2D3D".to_string(),
                text: "#F9FAFB".to_string(),
                text_muted: "#9CA3AF".to_string(),
                primary: "#4F46E5".to_string(),
                secondary: "#7C3AED".to_string(),
                accent: "#EC4899".to_string(),
                seasonal_primary: "#4F46E5".to_string(),
                seasonal_secondary: "#7C3AED".to_string(),
                seasonal_accent: "#EC4899".to_string(),
            }
        }
    }
}

pub fn get_spring_theme(mode: Mode) -> Theme {
    let mut theme = get_default_theme(mode);
    theme.name = match mode {
        Mode::Light => "Light Spring".to_string(),
        Mode::Dark => "Dark Spring".to_string(),
    };
    theme.season = Some(Season::Spring);
    theme.colors.seasonal_primary = "#10B981".to_string();    // Emerald
    theme.colors.seasonal_secondary = "#059669".to_string();  // Emerald darker
    theme.colors.seasonal_accent = "#34D399".to_string();     // Emerald lighter
    theme
}

pub fn get_summer_theme(mode: Mode) -> Theme {
    let mut theme = get_default_theme(mode);
    theme.name = match mode {
        Mode::Light => "Light Summer".to_string(),
        Mode::Dark => "Dark Summer".to_string(),
    };
    theme.season = Some(Season::Summer);
    theme.colors.seasonal_primary = "#F59E0B".to_string();    // Amber
    theme.colors.seasonal_secondary = "#D97706".to_string();  // Amber darker
    theme.colors.seasonal_accent = "#FBBF24".to_string();     // Amber lighter
    theme
}

mod router;
mod provider;
mod utils;

pub use router::ThemeRouter;
pub use provider::ThemeProvider;
pub use utils::{get_system_mode, get_theme_variant};

lazy_static::lazy_static! {
    pub static ref SPRING_LIGHT_THEME: Theme = get_spring_theme(Mode::Light);
    pub static ref SPRING_DARK_THEME: Theme = get_spring_theme(Mode::Dark);
    pub static ref SUMMER_LIGHT_THEME: Theme = get_summer_theme(Mode::Light);
    pub static ref SUMMER_DARK_THEME: Theme = get_summer_theme(Mode::Dark);
    pub static ref DEFAULT_LIGHT_THEME: Theme = get_default_theme(Mode::Light);
    pub static ref DEFAULT_DARK_THEME: Theme = get_default_theme(Mode::Dark);
} 