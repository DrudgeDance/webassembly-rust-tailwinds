use leptos::*;
use crate::theme::{BaseTheme, Mode, Theme};
use super::themes;

#[component]
pub fn MainContent(
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
        <main 
            class="flex-grow flex flex-col items-center justify-center p-8"
            style=move || format!(
                "background-color: {}; color: {}; border-top: 1px solid {}; box-shadow: inset 0 2px 4px {}",
                theme_memo.get().colors.background,
                theme_memo.get().colors.text,
                theme_memo.get().colors.border,
                theme_memo.get().colors.shadow,
            )
        >
            {children()}
        </main>
    }
} 