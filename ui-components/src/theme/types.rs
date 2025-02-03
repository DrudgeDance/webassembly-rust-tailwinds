#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Season {
    Spring,
    Summer,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Light,
    Dark,
}

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub background: String,
    pub surface: String,
    pub text: String,
    pub text_muted: String,
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub seasonal_primary: String,
    pub seasonal_secondary: String,
    pub seasonal_accent: String,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub season: Season,
    pub mode: Mode,
    pub colors: ThemeColors,
} 