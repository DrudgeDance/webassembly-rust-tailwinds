use leptos::*;
use leptos::ev::MouseEvent;
use crate::theme::{BaseTheme, Mode, Theme};

pub fn get_theme_colors(theme_opt: Option<Theme>, current_theme: Signal<BaseTheme>) -> (&'static str, &'static str, &'static str) {
    let mode = current_theme.with_untracked(|t| t.mode);
    match (mode, theme_opt) {
        (Mode::Light, None) => ("bg-slate-100", "text-slate-800", "hover:bg-slate-200"),
        (Mode::Dark, None) => ("bg-slate-800", "text-slate-100", "hover:bg-slate-700"),
        (Mode::Light, Some(Theme::Spring)) => ("bg-green-100", "text-green-800", "hover:bg-green-200"),
        (Mode::Dark, Some(Theme::Spring)) => ("bg-green-900", "text-green-100", "hover:bg-green-800"),
        (Mode::Light, Some(Theme::Summer)) => ("bg-amber-100", "text-amber-800", "hover:bg-amber-200"),
        (Mode::Dark, Some(Theme::Summer)) => ("bg-amber-900", "text-amber-100", "hover:bg-amber-800"),
    }
}

pub fn create_theme_handlers(
    theme: Signal<BaseTheme>,
    set_theme: WriteSignal<BaseTheme>,
    on_theme_change: Callback<BaseTheme>,
    set_is_open: WriteSignal<bool>,
) -> (impl Fn(Option<Theme>) + Clone, impl Fn(MouseEvent) + Clone) {
    let handle_theme_select = move |theme_opt: Option<Theme>| {
        let current = theme.get();
        let new_theme = BaseTheme {
            theme: theme_opt,
            ..current.clone()
        };
        set_theme.set(new_theme.clone());
        on_theme_change.call(new_theme);
        set_is_open.set(false);
    };

    let handle_mode_toggle = move |ev: MouseEvent| {
        ev.stop_propagation();  // Prevent dropdown from opening
        let current = theme.get();
        let new_theme = BaseTheme {
            mode: if current.mode == Mode::Light { Mode::Dark } else { Mode::Light },
            ..current.clone()
        };
        set_theme.set(new_theme.clone());
        on_theme_change.call(new_theme);
        set_is_open.set(false);  // Close the dropdown when toggling mode
    };

    (handle_theme_select, handle_mode_toggle)
}

pub fn get_theme_icon(theme: Option<Theme>) -> &'static str {
    match theme {
        None => "🎨",
        Some(Theme::Spring) => "🌱",
        Some(Theme::Summer) => "🌞",
    }
}

pub fn get_mode_icon(mode: Mode) -> &'static str {
    match mode {
        Mode::Light => "☀️",
        Mode::Dark => "🌙",
    }
} 