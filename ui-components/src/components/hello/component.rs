use leptos::*;
use crate::theme::{Mode, Season, Theme};
use super::themes;
use crate::components::theme_selector::ThemeSelector;

#[component]
pub fn Hello(
    #[prop(into)] message: Signal<String>,
    #[prop(into)] mode: Signal<Mode>,
    #[prop(into)] season: Signal<Option<Season>>,
    #[prop(into)] theme: Signal<Theme>,
    #[prop(into)] on_theme_change: Callback<(Mode, Option<Season>)>,
) -> impl IntoView {
    let theme_memo = create_memo(move |_| {
        match (mode.get(), season.get()) {
            (Mode::Light, Some(Season::Spring)) => themes::get_light_spring(),
            (Mode::Dark, Some(Season::Spring)) => themes::get_dark_spring(),
            (Mode::Light, Some(Season::Summer)) => themes::get_light_summer(),
            (Mode::Dark, Some(Season::Summer)) => themes::get_dark_summer(),
            (Mode::Light, None) => themes::get_light_default(),
            (Mode::Dark, None) => themes::get_dark_default(),
        }
    });

    view! {
        <div 
            class="flex flex-col h-screen"
            style=move || format!(
                "background-color: {}; color: {}; box-shadow: 0 4px 6px {}", 
                theme_memo.get().colors.background,
                theme_memo.get().colors.text,
                theme_memo.get().colors.shadow
            )
        >
            <nav 
                class="p-4"
                style=move || format!(
                    "background-color: {}",
                    theme_memo.get().colors.surface
                )
            >
                <div class="flex justify-between items-center max-w-4xl mx-auto">
                    <h1 class="text-2xl font-bold">
                        "My App"
                    </h1>
                    <ThemeSelector
                        current_theme=theme
                        on_theme_change=on_theme_change
                    />
                </div>
            </nav>
            <main class="flex-grow flex flex-col items-center justify-center space-y-8">
                <h2 class="text-4xl font-bold">
                    {move || message.get()}
                </h2>
                <p class="text-text-muted">
                    {move || format!("Current theme: {} - {}", 
                        if mode.get() == Mode::Light { "Light" } else { "Dark" },
                        season.get().map(|s| format!("{:?}", s)).unwrap_or_else(|| "Default".to_string())
                    )}
                </p>
            </main>
        </div>
    }
} 