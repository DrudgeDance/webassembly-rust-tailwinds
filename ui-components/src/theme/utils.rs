use super::{Theme, Mode};
use web_sys::window;

pub fn get_system_mode() -> Mode {
    window()
        .and_then(|window| window.match_media("(prefers-color-scheme: dark)").ok())
        .flatten()
        .map(|query| query.matches())
        .map(|is_dark| if is_dark { Mode::Dark } else { Mode::Light })
        .unwrap_or(Mode::Light)
}

pub fn get_theme_variant(base_theme: &Theme, mode: Mode) -> Theme {
    if base_theme.mode == mode {
        base_theme.clone()
    } else {
        match mode {
            Mode::Light => super::themes::get_spring_light_theme(),
            Mode::Dark => super::themes::get_spring_dark_theme(),
        }
    }
} 