#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Spring,
    Summer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Light,
    Dark,
}

// Base trait for component-specific colors
pub trait ComponentColors {
    fn from_theme(theme: &BaseTheme) -> Self;
}

// Component-specific theme
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentTheme<T: ComponentColors> {
    pub name: String,
    pub mode: Mode,
    pub theme: Option<Theme>,
    pub colors: T,
}

// Base theme for the entire application
#[derive(Debug, Clone)]
pub struct BaseTheme {
    pub name: String,
    pub mode: Mode,
    pub theme: Option<Theme>,
    pub colors: ThemeColors,
}

impl BaseTheme {
    pub fn default_light() -> Self {
        super::get_default_theme(Mode::Light)
    }

    pub fn default_dark() -> Self {
        super::get_default_theme(Mode::Dark)
    }

    pub fn spring_light() -> Self {
        super::get_spring_theme(Mode::Light)
    }

    pub fn spring_dark() -> Self {
        super::get_spring_theme(Mode::Dark)
    }

    pub fn summer_light() -> Self {
        super::get_summer_theme(Mode::Light)
    }

    pub fn summer_dark() -> Self {
        super::get_summer_theme(Mode::Dark)
    }
}

// Colors for the base theme
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeColors {
    pub background: String,
    pub surface: String,
    pub text: String,
    pub text_muted: String,
    pub primary: String,
    pub secondary: String,
    pub accent: String,
} 