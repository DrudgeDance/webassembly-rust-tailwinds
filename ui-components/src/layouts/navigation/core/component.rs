use leptos::*;
use crate::theme::{BaseTheme, Mode, Theme};
use super::themes;
use crate::layouts::navbar::Navbar;

#[component]
pub fn Layout(
    #[prop(into)] theme: Signal<BaseTheme>,
    children: Children,
) -> impl IntoView {
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
        <div
            class="min-h-screen"
            style=move || format!(
                "background-color: {}; color: {}; box-shadow: 0 0 10px {}",
                theme_memo.get().colors.background,
                theme_memo.get().colors.text,
                theme_memo.get().colors.shadow,
            )
        >
            <Navbar theme=theme />
            <main class="flex-grow flex flex-col items-center justify-center space-y-8">
                {children()}
            </main>
        </div>
    }
} 