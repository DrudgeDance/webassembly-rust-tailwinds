use super::types::{Mode, Season};

pub struct ThemeRouter;

impl ThemeRouter {
    pub fn next_season(current_season: Option<Season>) -> Option<Season> {
        match current_season {
            None => Some(Season::Spring),
            Some(Season::Spring) => Some(Season::Summer),
            Some(Season::Summer) => None,
        }
    }

    pub fn previous_season(current_season: Option<Season>) -> Option<Season> {
        match current_season {
            None => Some(Season::Summer),
            Some(Season::Spring) => None,
            Some(Season::Summer) => Some(Season::Spring),
        }
    }

    pub fn toggle_mode(current_mode: Mode) -> Mode {
        match current_mode {
            Mode::Light => Mode::Dark,
            Mode::Dark => Mode::Light,
        }
    }
} 