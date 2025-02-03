use crate::theme::{Mode, Season, Theme};

pub fn get_system_mode() -> Mode {
    // TODO: Implement actual system theme detection
    Mode::Light
}

pub fn get_theme_variant(theme: &Theme, new_mode: Mode) -> Theme {
    let season = theme.season;
    match (new_mode, season) {
        (Mode::Light, Some(Season::Spring)) => Theme::spring_light(),
        (Mode::Dark, Some(Season::Spring)) => Theme::spring_dark(),
        (Mode::Light, Some(Season::Summer)) => Theme::summer_light(),
        (Mode::Dark, Some(Season::Summer)) => Theme::summer_dark(),
        (Mode::Light, None) => Theme::default_light(),
        (Mode::Dark, None) => Theme::default_dark(),
    }
} 