use leptos::*;
use crate::theme::{BaseTheme, Theme, Mode};

#[component]
pub fn ThemeSelectorDropdown(
    #[prop(into)] theme: Signal<BaseTheme>,
    #[prop(into)] is_open: Signal<bool>,
    #[prop(into)] on_theme_select: Callback<Option<Theme>>,
    #[prop(into)] on_preview: Callback<Option<Theme>>,
) -> impl IntoView {
    let theme_button = move |theme_opt: Option<Theme>, label: &'static str, icon: &'static str| {
        let (bg, text, hover) = super::get_theme_colors(theme_opt, theme);
        let base_classes = "w-full px-4 py-1.25 rounded-lg text-left transition-all duration-200 flex items-center gap-2";
        
        view! {
            <button
                class={format!("{} {} {} {}", base_classes, bg, text, hover)}
                on:click=move |_| on_theme_select.call(theme_opt)
                on:mouseenter=move |_| on_preview.call(theme_opt)
                on:mouseleave=move |_| on_preview.call(None)
            >
                <span class="text-lg">{icon}</span>
                <span class="flex-grow">{label}</span>
                <span class="text-sm ml-2">
                    {move || {
                        let current_theme = theme.get().theme;
                        if current_theme == theme_opt { "✔" } else { "" }
                    }}
                </span>
            </button>
        }
    };

    view! {
        <div
            class="absolute w-48 p-2 rounded-xl shadow-lg space-y-2 transition-all duration-200 bg-surface"
            class=move || {
                format!("{}", if is_open.get() { "block" } else { "hidden" })
            }
            style="min-width: 200px; position: absolute; right: 0; top: 100%; transform-origin: top right; z-index: 50;"
        >
            {theme_button(None, "Default", "🎨")}
            {theme_button(Some(Theme::Spring), "Spring", "🌱")}
            {theme_button(Some(Theme::Summer), "Summer", "🌞")}
        </div>
    }
} 