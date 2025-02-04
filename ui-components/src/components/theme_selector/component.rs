use leptos::*;
use crate::theme::{Mode, Theme, BaseTheme};
use web_sys::KeyboardEvent;
use super::themes;

#[component]
pub fn ThemeSelector(
    #[prop(into)] current_theme: Signal<BaseTheme>,
    #[prop(into)] on_theme_change: Callback<(Mode, Option<Theme>)>,
) -> impl IntoView {
    let theme_memo = create_memo(move |_| {
        match (current_theme.get().mode, current_theme.get().theme) {
            (Mode::Light, Some(Theme::Spring)) => themes::get_light_spring(),
            (Mode::Dark, Some(Theme::Spring)) => themes::get_dark_spring(),
            (Mode::Light, Some(Theme::Summer)) => themes::get_light_summer(),
            (Mode::Dark, Some(Theme::Summer)) => themes::get_dark_summer(),
            (Mode::Light, None) => themes::get_light_default(),
            (Mode::Dark, None) => themes::get_dark_default(),
        }
    });

    let handle_mode_toggle = move |_| {
        let current = current_theme.get();
        let new_mode = match current.mode {
            Mode::Light => Mode::Dark,
            Mode::Dark => Mode::Light,
        };
        on_theme_change.call((new_mode, current.theme));
    };

    let handle_theme_change = move |new_theme: Option<Theme>| {
        let current = current_theme.get();
        on_theme_change.call((current.mode, new_theme));
    };

    // Set up global keyboard shortcuts
    let switch_theme = on_theme_change.clone();
    window_event_listener(ev::keydown, move |ev: KeyboardEvent| {
        if ev.shift_key() {
            match ev.key().to_lowercase().as_str() {
                "s" => {
                    // Toggle light/dark mode
                    ev.prevent_default();
                    let current = current_theme.get();
                    let new_mode = match current.mode {
                        Mode::Light => Mode::Dark,
                        Mode::Dark => Mode::Light,
                    };
                    switch_theme.call((new_mode, current.theme));
                },
                "x" => {
                    // Next theme
                    ev.prevent_default();
                    let current = current_theme.get();
                    let new_theme = match current.theme {
                        None => Some(Theme::Spring),
                        Some(Theme::Spring) => Some(Theme::Summer),
                        Some(Theme::Summer) => None,
                    };
                    switch_theme.call((current.mode, new_theme));
                },
                "w" => {
                    // Previous theme
                    ev.prevent_default();
                    let current = current_theme.get();
                    let new_theme = match current.theme {
                        None => Some(Theme::Summer),
                        Some(Theme::Spring) => None,
                        Some(Theme::Summer) => Some(Theme::Spring),
                    };
                    switch_theme.call((current.mode, new_theme));
                },
                _ => (),
            }
        }
    });

    let button_style = move || format!(
        "background-color: {}; color: {}; border-color: {}; transition-property: background-color; transition-duration: 200ms; &:hover {{ background-color: {}; }}",
        theme_memo.get().colors.button_bg,
        theme_memo.get().colors.button_text,
        theme_memo.get().colors.button_border,
        theme_memo.get().colors.hover_bg,
    );

    let active_button_style = move |is_active: bool| {
        if is_active {
            format!(
                "background-color: {}; color: {};",
                theme_memo.get().colors.active_bg,
                theme_memo.get().colors.active_text,
            )
        } else {
            button_style()
        }
    };

    view! {
        <div class="flex items-center space-x-4">
            <div class="flex flex-col space-y-2">
                <button
                    class="px-4 py-2 rounded-lg"
                    style=button_style
                    on:click=handle_mode_toggle
                >
                    {move || match current_theme.get().mode {
                        Mode::Light => "Switch to Dark Mode",
                        Mode::Dark => "Switch to Light Mode",
                    }}
                </button>
                <div class="flex space-x-2">
                    <button
                        class="px-4 py-2 rounded-lg"
                        style=move || active_button_style(current_theme.get().theme.is_none())
                        on:click=move |_| handle_theme_change(None)
                    >
                        "Default"
                    </button>
                    <button
                        class="px-4 py-2 rounded-lg"
                        style=move || active_button_style(current_theme.get().theme == Some(Theme::Spring))
                        on:click=move |_| handle_theme_change(Some(Theme::Spring))
                    >
                        "Spring"
                    </button>
                    <button
                        class="px-4 py-2 rounded-lg"
                        style=move || active_button_style(current_theme.get().theme == Some(Theme::Summer))
                        on:click=move |_| handle_theme_change(Some(Theme::Summer))
                    >
                        "Summer"
                    </button>
                </div>
                <div class="text-sm text-text-muted text-center">
                    {move || format!("Current theme: {} - {}", 
                        match current_theme.get().mode {
                            Mode::Light => "Light",
                            Mode::Dark => "Dark",
                        },
                        match current_theme.get().theme {
                            None => "Default",
                            Some(Theme::Spring) => "Spring",
                            Some(Theme::Summer) => "Summer",
                        }
                    )}
                </div>
            </div>
        </div>
    }
} 