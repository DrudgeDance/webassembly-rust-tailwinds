use leptos::*;
use crate::theme::{BaseTheme, Mode, Theme};
use super::themes::{self, LayoutTheme};

pub fn create_theme_memo(theme: Signal<BaseTheme>) -> Memo<LayoutTheme> {
    create_memo(move |_| {
        match (theme.get().mode, theme.get().theme) {
            (Mode::Light, None) => themes::get_light_default(),
            (Mode::Dark, None) => themes::get_dark_default(),
            (Mode::Light, Some(Theme::Spring)) => themes::get_light_spring(),
            (Mode::Dark, Some(Theme::Spring)) => themes::get_dark_spring(),
            (Mode::Light, Some(Theme::Summer)) => themes::get_light_summer(),
            (Mode::Dark, Some(Theme::Summer)) => themes::get_dark_summer(),
        }
    })
} 