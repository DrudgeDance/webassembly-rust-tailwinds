use leptos::*;
use crate::theme::{BaseTheme, Mode, Theme};

#[component]
pub fn ThemeSelector(
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    let set_theme = use_context::<WriteSignal<BaseTheme>>()
        .expect("ThemeProvider should be present");
    
    let handle_mode_change = move |_| {
        let current = theme.get();
        let new_mode = if current.mode == Mode::Light { Mode::Dark } else { Mode::Light };
        set_theme.set(BaseTheme {
            mode: new_mode,
            ..current
        });
    };

    let handle_theme_change = move |new_theme: Option<Theme>| {
        let current = theme.get();
        set_theme.set(BaseTheme {
            theme: new_theme,
            ..current
        });
    };

    view! {
        <div class="flex items-center space-x-4">
            <button
                class="p-2 rounded hover:bg-opacity-10 hover:bg-current"
                on:click=handle_mode_change
            >
                {move || if theme.get().mode == Mode::Light { "🌙" } else { "☀️" }}
            </button>
            <select
                class="bg-transparent border rounded p-1"
                style=move || format!(
                    "border-color: {}",
                    theme.get().colors.text_muted
                )
                on:change=move |ev| {
                    let value = event_target_value(&ev);
                    let theme = match value.as_str() {
                        "default" => None,
                        "spring" => Some(Theme::Spring),
                        "summer" => Some(Theme::Summer),
                        _ => None,
                    };
                    handle_theme_change(theme);
                }
            >
                <option value="default">Default</option>
                <option value="spring">Spring</option>
                <option value="summer">Summer</option>
            </select>
        </div>
    }
} 