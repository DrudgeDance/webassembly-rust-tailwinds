use leptos::*;
use crate::theme::{BaseTheme, Mode, Theme};
use super::themes;
use crate::layouts::Base;

const DEFAULT_MESSAGE: &str = "Hello, World!";

#[component]
pub fn Hello(
    #[prop(into)] theme: Signal<BaseTheme>,
    #[prop(into, optional)] message: Option<Signal<String>>,
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

    let display_message = create_memo(move |_| {
        message.map(|m| m.get()).unwrap_or_else(|| DEFAULT_MESSAGE.to_string())
    });

    view! {
        <Base theme=theme>
            <div class="flex items-center justify-center">
                <div
                    class="p-6 rounded-lg"
                    style=move || format!(
                        "background-color: {}; color: {}; border: 1px solid {}; box-shadow: 0 4px 6px {}",
                        theme_memo.get().colors.background,
                        theme_memo.get().colors.text,
                        theme_memo.get().colors.border,
                        theme_memo.get().colors.shadow,
                    )
                >
                    <h1 class="text-2xl font-bold">{display_message}</h1>
                    <p
                        class="mt-2"
                        style=move || format!("color: {}", theme_memo.get().colors.text_muted)
                    >
                        "Welcome to our themed application!"
                    </p>
                </div>
            </div>
        </Base>
    }
} 