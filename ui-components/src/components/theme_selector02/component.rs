use leptos::*;
use leptos::ev::MouseEvent;
use crate::theme::{BaseTheme, Mode, Theme};

#[component]
pub fn ThemeSelector02(
    #[prop(into)] theme: Signal<BaseTheme>,
    #[prop(into)] on_theme_change: Callback<BaseTheme>,
) -> impl IntoView {
    let set_theme = use_context::<WriteSignal<BaseTheme>>()
        .expect("ThemeProvider should be present");
    
    let (is_open, set_is_open) = create_signal(false);
    let (active_preview, set_active_preview) = create_signal::<Option<Theme>>(None);

    let handle_theme_select = move |theme_opt: Option<Theme>| {
        let current = theme.get();
        let new_theme = BaseTheme {
            theme: theme_opt,
            ..current.clone()
        };
        set_theme.set(new_theme.clone());
        on_theme_change.call(new_theme);
        set_is_open.set(false);
    };

    let handle_mode_toggle = move |ev: MouseEvent| {
        ev.stop_propagation();  // Prevent dropdown from opening
        let current = theme.get();
        let new_theme = BaseTheme {
            mode: if current.mode == Mode::Light { Mode::Dark } else { Mode::Light },
            ..current.clone()
        };
        set_theme.set(new_theme.clone());
        on_theme_change.call(new_theme);
    };

    let get_theme_colors = move |theme_opt: Option<Theme>| {
        let mode = theme.with_untracked(|t| t.mode);  // Use with_untracked since we don't need reactivity for mode
        match (mode, theme_opt) {
            (Mode::Light, None) => ("bg-slate-100", "text-slate-800", "hover:bg-slate-200"),
            (Mode::Dark, None) => ("bg-slate-800", "text-slate-100", "hover:bg-slate-700"),
            (Mode::Light, Some(Theme::Spring)) => ("bg-green-100", "text-green-800", "hover:bg-green-200"),
            (Mode::Dark, Some(Theme::Spring)) => ("bg-green-900", "text-green-100", "hover:bg-green-800"),
            (Mode::Light, Some(Theme::Summer)) => ("bg-amber-100", "text-amber-800", "hover:bg-amber-200"),
            (Mode::Dark, Some(Theme::Summer)) => ("bg-amber-900", "text-amber-100", "hover:bg-amber-800"),
        }
    };

    let theme_button = move |theme_opt: Option<Theme>, label: &'static str, icon: &'static str| {
        let (bg, text, hover) = get_theme_colors(theme_opt);
        let base_classes = "w-full px-4 py-2 rounded-lg text-left transition-all duration-200 flex items-center gap-2";
        
        view! {
            <button
                class={format!("{} {} {} {}", base_classes, bg, text, hover)}
                on:click=move |_| handle_theme_select(theme_opt)
                on:mouseenter=move |_| set_active_preview.set(theme_opt)
                on:mouseleave=move |_| set_active_preview.set(None)
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
        <div class="relative z-50">
            <button
                class="flex items-center gap-2 px-3 py-2 rounded-lg transition-all duration-200"
                class=move || {
                    let (bg, text, hover) = get_theme_colors(theme.get().theme);
                    format!("{} {} {}", bg, text, hover)
                }
                on:click=move |_| set_is_open.update(|v| *v = !*v)
            >
                <span class="text-lg">{move || match theme.get().theme {
                    None => "🎨",
                    Some(Theme::Spring) => "🌱",
                    Some(Theme::Summer) => "🌞",
                }}</span>
                <span>{move || match theme.get().theme {
                    None => "Default",
                    Some(Theme::Spring) => "Spring",
                    Some(Theme::Summer) => "Summer",
                }}</span>
                <span 
                    class="text-lg ml-2 border-l pl-2 cursor-pointer hover:opacity-80" 
                    style="border-color: currentColor"
                    on:click=handle_mode_toggle
                >
                    {move || if theme.get().mode == Mode::Light { "☀️" } else { "🌙" }}
                </span>
                <span class="text-sm transition-transform duration-200 ml-1" 
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
                    let (bg, _, _) = get_theme_colors(preview);
                    format!("{}", bg)
                }
                style=move || format!(
                    "opacity: {}; transform: scaleX({})",
                    if active_preview.get().is_some() { "1" } else { "0" },
                    if active_preview.get().is_some() { "1" } else { "0.8" }
                )
            ></div>

            // Dropdown menu
            <div
                class="absolute left-0 right-0 mt-3 p-2 rounded-lg shadow-lg space-y-2 transition-all duration-200"
                class=move || {
                    let (bg, _, _) = get_theme_colors(None);
                    format!("{}", bg)
                }
                style=move || format!(
                    "opacity: {}; transform: translateY({}); pointer-events: {};",
                    if is_open.get() { "1" } else { "0" },
                    if is_open.get() { "0" } else { "-8px" },
                    if is_open.get() { "auto" } else { "none" }
                )
            >
                {theme_button(None, "Default", "🎨")}
                {theme_button(Some(Theme::Spring), "Spring", "🌱")}
                {theme_button(Some(Theme::Summer), "Summer", "🌞")}
            </div>
        </div>
    }
} 