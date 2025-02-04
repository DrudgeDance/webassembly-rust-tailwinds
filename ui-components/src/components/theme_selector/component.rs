use leptos::*;
use crate::theme::{BaseTheme, Mode, Theme};
use super::themes;

#[component]
pub fn ThemeSelector(
    #[prop(into)] theme: Signal<BaseTheme>,
    #[prop(into)] on_theme_change: Callback<BaseTheme>,
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

    let handle_mode_change = move |_| {
        let current = theme.get();
        let new_mode = if current.mode == Mode::Light { Mode::Dark } else { Mode::Light };
        let new_theme = BaseTheme {
            mode: new_mode,
            ..current.clone()
        };
        set_theme.set(new_theme.clone());
        on_theme_change.call(new_theme);
    };

    let handle_theme_change = move |new_theme: Option<Theme>| {
        let current = theme.get();
        let new_theme_base = BaseTheme {
            theme: new_theme,
            ..current.clone()
        };
        set_theme.set(new_theme_base.clone());
        on_theme_change.call(new_theme_base);
    };

    view! {
        <div class="flex items-center space-x-4">
            <button
                class="p-2 rounded transition-colors duration-200"
                style=move || format!(
                    "background-color: {}; color: {}; border: 1px solid {}; &:hover {{ background-color: {}; }}",
                    theme_memo.get().colors.button_bg,
                    theme_memo.get().colors.button_text,
                    theme_memo.get().colors.button_border,
                    theme_memo.get().colors.button_hover_bg,
                )
                on:click=handle_mode_change
            >
                <span style=move || format!("color: {}", theme_memo.get().colors.icon_color)>
                    {move || if theme.get().mode == Mode::Light { "🌙" } else { "☀️" }}
                </span>
            </button>
            <select
                class="p-2 rounded transition-colors duration-200"
                style=move || format!(
                    "background-color: {}; color: {}; border: 1px solid {}; &:hover {{ background-color: {}; }}",
                    theme_memo.get().colors.select_bg,
                    theme_memo.get().colors.select_text,
                    theme_memo.get().colors.select_border,
                    theme_memo.get().colors.select_hover_bg,
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