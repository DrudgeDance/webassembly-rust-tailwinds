use leptos::*;
use crate::theme::{BaseTheme, Theme};

#[component]
pub fn ThemeSelectorDropdown(
    #[prop(into)] theme: Signal<BaseTheme>,
    #[prop(into)] is_open: Signal<bool>,
    #[prop(into)] on_theme_select: Callback<Option<Theme>>,
    #[prop(into)] on_preview: Callback<Option<Theme>>,
) -> impl IntoView {
    let theme_button = move |theme_opt: Option<Theme>, label: &'static str, icon: &'static str| {
        let (bg, text, hover) = super::get_theme_colors(theme_opt, theme);
        let base_classes = "w-full px-3 py-1.25 rounded-lg transition-all duration-200 flex items-center gap-1";
        
        view! {
            <button
                class={format!("{} {} {} {}", base_classes, bg, text, hover)}
                on:click=move |_| on_theme_select.call(theme_opt)
                on:mouseenter=move |_| on_preview.call(theme_opt)
                on:mouseleave=move |_| on_preview.call(None)
            >
                <span class="text-lg shrink-0 w-7">{icon}</span>
                <span class="flex-1 min-w-0">{label}</span>
                <span class="shrink-0 w-5 text-center">
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
            class="absolute p-2 rounded-xl shadow-lg space-y-2 transition-all duration-200 bg-surface min-w-fit"
            class=move || {
                format!("{}", if is_open.get() { "block" } else { "hidden" })
            }
            style="position: absolute; right: 0; top: 100%; transform-origin: top right; z-index: 50;"
        >
            {theme_button(None, "Default", "🎨")}
            {theme_button(Some(Theme::Spring), "Spring", "🌱")}
            {theme_button(Some(Theme::Summer), "Summer", "🌞")}
        </div>
    }
} 