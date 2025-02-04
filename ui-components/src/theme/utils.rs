use crate::theme::{Mode, Theme, BaseTheme};

pub fn get_system_mode() -> Mode {
    // TODO: Implement actual system theme detection
    Mode::Light
}

pub fn get_theme_variant(theme: &BaseTheme, new_mode: Mode) -> BaseTheme {
    let theme = theme.theme;
    match (new_mode, theme) {
        (Mode::Light, Some(Theme::Spring)) => BaseTheme::spring_light(),
        (Mode::Dark, Some(Theme::Spring)) => BaseTheme::spring_dark(),
        (Mode::Light, Some(Theme::Summer)) => BaseTheme::summer_light(),
        (Mode::Dark, Some(Theme::Summer)) => BaseTheme::summer_dark(),
        (Mode::Light, None) => BaseTheme::default_light(),
        (Mode::Dark, None) => BaseTheme::default_dark(),
    }
} 