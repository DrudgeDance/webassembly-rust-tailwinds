use super::types::{Mode, Theme};

pub struct ThemeRouter;

impl ThemeRouter {
    pub fn next_theme(current_theme: Option<Theme>) -> Option<Theme> {
        match current_theme {
            None => Some(Theme::Spring),
            Some(Theme::Spring) => Some(Theme::Summer),
            Some(Theme::Summer) => None,
        }
    }

    pub fn previous_theme(current_theme: Option<Theme>) -> Option<Theme> {
        match current_theme {
            None => Some(Theme::Summer),
            Some(Theme::Spring) => None,
            Some(Theme::Summer) => Some(Theme::Spring),
        }
    }

    pub fn toggle_mode(current_mode: Mode) -> Mode {
        match current_mode {
            Mode::Light => Mode::Dark,
            Mode::Dark => Mode::Light,
        }
    }
} 