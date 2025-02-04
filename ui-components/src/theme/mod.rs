mod types;
mod router;
mod provider;
mod utils;

pub use types::*;

pub fn get_default_theme(mode: Mode) -> BaseTheme {
    match mode {
        Mode::Light => BaseTheme {
            name: "Light Default".to_string(),
            mode: Mode::Light,
            theme: None,
            colors: ThemeColors {
                background: "#F8FAF8".to_string(),
                surface: "#FFFFFF".to_string(),
                text: "#111827".to_string(),
                text_muted: "#6B7280".to_string(),
                primary: "#4F46E5".to_string(),
                secondary: "#7C3AED".to_string(),
                accent: "#EC4899".to_string(),
            }
        },
        Mode::Dark => BaseTheme {
            name: "Dark Default".to_string(),
            mode: Mode::Dark,
            theme: None,
            colors: ThemeColors {
                background: "#1A1C2A".to_string(),
                surface: "#2D2D3D".to_string(),
                text: "#F9FAFB".to_string(),
                text_muted: "#9CA3AF".to_string(),
                primary: "#4F46E5".to_string(),
                secondary: "#7C3AED".to_string(),
                accent: "#EC4899".to_string(),
            }
        }
    }
}

pub fn get_spring_theme(mode: Mode) -> BaseTheme {
    let mut theme = get_default_theme(mode);
    theme.name = match mode {
        Mode::Light => "Light Spring".to_string(),
        Mode::Dark => "Dark Spring".to_string(),
    };
    theme.theme = Some(Theme::Spring);
    theme.colors.primary = "#10B981".to_string();    // Emerald
    theme.colors.secondary = "#059669".to_string();  // Emerald darker
    theme.colors.accent = "#34D399".to_string();     // Emerald lighter
    theme
}

pub fn get_summer_theme(mode: Mode) -> BaseTheme {
    let mut theme = get_default_theme(mode);
    theme.name = match mode {
        Mode::Light => "Light Summer".to_string(),
        Mode::Dark => "Dark Summer".to_string(),
    };
    theme.theme = Some(Theme::Summer);
    theme.colors.primary = "#F59E0B".to_string();    // Amber
    theme.colors.secondary = "#D97706".to_string();  // Amber darker
    theme.colors.accent = "#FBBF24".to_string();     // Amber lighter
    theme
}

lazy_static::lazy_static! {
    pub static ref SPRING_LIGHT_THEME: BaseTheme = get_spring_theme(Mode::Light);
    pub static ref SPRING_DARK_THEME: BaseTheme = get_spring_theme(Mode::Dark);
    pub static ref SUMMER_LIGHT_THEME: BaseTheme = get_summer_theme(Mode::Light);
    pub static ref SUMMER_DARK_THEME: BaseTheme = get_summer_theme(Mode::Dark);
    pub static ref DEFAULT_LIGHT_THEME: BaseTheme = get_default_theme(Mode::Light);
    pub static ref DEFAULT_DARK_THEME: BaseTheme = get_default_theme(Mode::Dark);
} 