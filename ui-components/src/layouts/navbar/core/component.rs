use leptos::*;
use crate::theme::{BaseTheme, Mode, Theme};
use super::themes;
use crate::components::theme_selector::ThemeSelector;

#[component]
pub fn Navbar(
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    let set_theme = use_context::<WriteSignal<BaseTheme>>()
        .expect("ThemeProvider should be present");

    let theme_memo = create_memo(move |_| {
        match (theme.get().mode, theme.get().theme) {
            (Mode::Light, None) => themes::get_light_default(),
            (Mode::Dark, None) => themes::get_dark_default(),
            (Mode::Light, Some(Theme::Spring)) => themes::get_light_spring(),
            (Mode::Dark, Some(Theme::Spring)) => themes::get_dark_spring(),
            (Mode::Light, Some(Theme::Summer)) => themes::get_light_summer(),
            (Mode::Dark, Some(Theme::Summer)) => themes::get_dark_summer(),
        }
    });

    view! {
        <nav 
            class="p-4 flex justify-between items-center"
            style=move || format!(
                "background-color: {}; color: {}; border-bottom: 1px solid {}",
                theme_memo.get().colors.background,
                theme_memo.get().colors.text,
                theme_memo.get().colors.border,
            )
        >
            <h1 class="text-xl font-bold">My App</h1>
            <ThemeSelector
                theme=theme
                on_theme_change=move |new_theme| set_theme.set(new_theme)
            />
        </nav>
    }
} 