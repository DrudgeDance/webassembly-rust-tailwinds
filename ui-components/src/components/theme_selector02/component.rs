use leptos::*;
use leptos::html::Div;
use crate::theme::{BaseTheme, Theme};
use super::dropdown::ThemeSelectorDropdown;
use super::theme_switcher::{get_theme_colors, create_theme_handlers, get_theme_icon, get_mode_icon};
use super::click_handler::handle_click_outside;

#[component]
pub fn ThemeSelector02(
    #[prop(into)] theme: Signal<BaseTheme>,
    #[prop(into)] on_theme_change: Callback<BaseTheme>,
) -> impl IntoView {
    let set_theme = use_context::<WriteSignal<BaseTheme>>()
        .expect("ThemeProvider should be present");
    
    let (is_open, set_is_open) = create_signal(false);
    let (active_preview, set_active_preview) = create_signal::<Option<Theme>>(None);
    let container_ref = create_node_ref::<Div>();

    // Setup click outside listener
    window_event_listener(ev::click, move |e| {
        handle_click_outside(e, container_ref, set_is_open);
    });

    let (handle_theme_select, handle_mode_toggle) = create_theme_handlers(
        theme,
        set_theme,
        on_theme_change,
        set_is_open,
    );

    view! {
        <div 
            class="relative inline-flex flex-col"
            node_ref=container_ref
        >
            <button
                class="inline-flex items-center gap-2 px-3 py-2 rounded-lg transition-all duration-200"
                class=move || {
                    let (bg, text, hover) = get_theme_colors(theme.get().theme, theme);
                    format!("{} {} {}", bg, text, hover)
                }
                on:click=move |_| set_is_open.update(|v| *v = !*v)
            >
                <span 
                    class="text-lg cursor-pointer hover:opacity-80" 
                    on:click=handle_mode_toggle
                >
                    {move || get_mode_icon(theme.get().mode)}
                </span>
                <span class="mr-2"></span>
                <span 
                    class="border-l" 
                    style="border-color: currentColor"
                ></span>
                <span class="mx-2 text-lg">{move || get_theme_icon(theme.get().theme)}</span>
                <span class="text-sm transition-transform duration-200" 
                    class=move || if is_open.get() { "rotate-180" } else { "" }
                >
                    "▼"
                </span>
            </button>

            // Theme preview bar
            <div
                class="absolute left-0 right-0 h-1 -bottom-1 transition-all duration-200 rounded-full"
                class=move || {
                    let preview = active_preview.get();
                    let (bg, _, _) = get_theme_colors(preview, theme);
                    format!("{}", bg)
                }
                style=move || format!(
                    "opacity: {}; transform: scaleX({})",
                    if active_preview.get().is_some() { "1" } else { "0" },
                    if active_preview.get().is_some() { "1" } else { "0.8" }
                )
            ></div>

            <ThemeSelectorDropdown 
                theme=theme
                is_open=is_open
                on_theme_select=handle_theme_select
                on_preview=Callback::from(move |theme: Option<Theme>| set_active_preview.set(theme))
            />
        </div>
    }
} 