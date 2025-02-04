use leptos::*;
use crate::theme::BaseTheme;
use crate::components::theme_selector::ThemeSelector;

#[component]
pub fn Navbar(
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    let set_theme = use_context::<WriteSignal<BaseTheme>>()
        .expect("ThemeProvider should be present");

    view! {
        <nav 
            class="p-4 flex justify-between items-center"
            style=move || format!(
                "background-color: {}; color: {}; border-bottom: 1px solid {}",
                theme.get().colors.background,
                theme.get().colors.text,
                theme.get().colors.text_muted,
            )
        >
            <h1 class="text-xl font-bold">My App</h1>
            <ThemeSelector
                theme=theme
                on_theme_change=move |new_theme| set_theme.set(new_theme)
            />
        </nav>
    }
} 