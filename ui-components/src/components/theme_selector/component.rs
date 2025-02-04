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

    let (is_open, set_is_open) = create_signal(false);

    let handle_mode_toggle = move |_| {
        let current = theme.get();
        let new_mode = if current.mode == Mode::Light { Mode::Dark } else { Mode::Light };
        let new_theme = BaseTheme {
            mode: new_mode,
            ..current.clone()
        };
        set_theme.set(new_theme.clone());
        on_theme_change.call(new_theme);
    };

    let handle_theme_select = move |theme_opt| {
        let current = theme.get();
        let new_theme = BaseTheme {
            theme: theme_opt,
            ..current.clone()
        };
        set_theme.set(new_theme.clone());
        on_theme_change.call(new_theme);
        set_is_open.set(false);
    };

    let get_mode_icon = move |mode: Mode| {
        match mode {
            Mode::Light => "☀️",
            Mode::Dark => "🌙",
        }
    };

    let get_theme_icon = move |theme_opt: Option<Theme>| {
        match theme_opt {
            None => "🎨",
            Some(Theme::Spring) => "🌸",
            Some(Theme::Summer) => "🌞",
        }
    };

    let get_hover_style = move |is_selected: bool| {
        format!(
            "color: {}; {} hover: {{ background-color: {}; }}",
            theme_memo.get().colors.select_text,
            if is_selected {
                format!("background-color: {};", theme_memo.get().colors.button_hover_bg)
            } else {
                String::new()
            },
            theme_memo.get().colors.button_hover_bg,
        )
    };

    view! {
        <div class="relative inline-flex rounded-lg border-2" style=move || format!(
            "border-color: {};",
            theme_memo.get().colors.button_border,
        )>
            // Mode toggle button
            <button
                class="px-4 py-2 transition-all duration-200 flex items-center gap-2"
                style=move || format!(
                    "background-color: {}; color: {};",
                    theme_memo.get().colors.button_bg,
                    theme_memo.get().colors.button_text,
                )
                on:click=handle_mode_toggle
            >
                <span>{move || format!("{} {}", 
                    get_mode_icon(theme.get().mode),
                    if theme.get().mode == Mode::Light { "Light" } else { "Dark" }
                )}</span>
            </button>

            // Theme selector dropdown
            <div class="relative">
                <button
                    class="px-4 py-2 transition-all duration-200 flex items-center gap-2 min-w-[140px]"
                    style=move || format!(
                        "background-color: {}; color: {};",
                        theme_memo.get().colors.button_bg,
                        theme_memo.get().colors.button_text,
                    )
                    on:click=move |_| set_is_open.update(|v| *v = !*v)
                >
                    <span>{move || {
                        let theme_type = theme.get().theme;
                        format!("{} {}", 
                            get_theme_icon(theme_type),
                            match theme_type {
                                None => "Default",
                                Some(Theme::Spring) => "Spring",
                                Some(Theme::Summer) => "Summer",
                            }
                        )
                    }}</span>
                    <span class="ml-auto text-sm" style=move || format!("color: {}", theme_memo.get().colors.button_text)>
                        {move || if is_open.get() { "▲" } else { "▼" }}
                    </span>
                </button>

                <div
                    class="absolute top-full right-0 mt-1 w-[140px] rounded-lg overflow-hidden transition-all duration-200 z-50"
                    style=move || format!(
                        "background-color: {}; border: 1px solid {}; box-shadow: 0 4px 6px rgba(0,0,0,0.1); {}",
                        theme_memo.get().colors.select_bg,
                        theme_memo.get().colors.select_border,
                        if is_open.get() { "opacity: 1; transform: translateY(0); pointer-events: auto;" } 
                        else { "opacity: 0; transform: translateY(-10px); pointer-events: none;" }
                    )
                >
                    <div class="p-2">
                        <button
                            class="w-full px-3 py-2 text-left transition-colors duration-200 rounded"
                            style=move || get_hover_style(theme.get().theme.is_none())
                            on:click=move |_| handle_theme_select(None)
                        >
                            "🎨 Default"
                        </button>
                        <button
                            class="w-full px-3 py-2 text-left transition-colors duration-200 rounded"
                            style=move || get_hover_style(theme.get().theme == Some(Theme::Spring))
                            on:click=move |_| handle_theme_select(Some(Theme::Spring))
                        >
                            "🌸 Spring"
                        </button>
                        <button
                            class="w-full px-3 py-2 text-left transition-colors duration-200 rounded"
                            style=move || get_hover_style(theme.get().theme == Some(Theme::Summer))
                            on:click=move |_| handle_theme_select(Some(Theme::Summer))
                        >
                            "🌞 Summer"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
} 