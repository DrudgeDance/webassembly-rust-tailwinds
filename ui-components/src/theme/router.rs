use super::types::{Theme, Season, Mode};
use super::themes::{
    get_spring_light_theme,
    get_spring_dark_theme,
    get_summer_light_theme,
    get_summer_dark_theme
};

pub struct ThemeRouter;

impl ThemeRouter {
    pub fn get_theme(season: Season, mode: Mode) -> Theme {
        match (season, mode) {
            (Season::Spring, Mode::Light) => get_spring_light_theme(),
            (Season::Spring, Mode::Dark) => get_spring_dark_theme(),
            (Season::Summer, Mode::Light) => get_summer_light_theme(),
            (Season::Summer, Mode::Dark) => get_summer_dark_theme(),
        }
    }

    pub fn next_season(current_season: Season) -> Season {
        match current_season {
            Season::Spring => Season::Summer,
            Season::Summer => Season::Spring,
        }
    }

    pub fn previous_season(current_season: Season) -> Season {
        match current_season {
            Season::Spring => Season::Summer,
            Season::Summer => Season::Spring,
        }
    }

    pub fn toggle_mode(current_mode: Mode) -> Mode {
        match current_mode {
            Mode::Light => Mode::Dark,
            Mode::Dark => Mode::Light,
        }
    }
} 