use leptos::*;
use leptos::logging::log;
use crate::theme::Theme;

#[component]
pub fn ThemeProvider(
    #[prop(into)] theme: Signal<Theme>,
    #[prop(into)] set_theme: WriteSignal<Theme>,
    children: Children,
) -> impl IntoView {
    // Provide theme context to children
    provide_context(theme);
    provide_context(set_theme);

    create_effect(move |_| {
        log!("Theme mode: {:?}, season: {:?}", theme.get().mode, theme.get().season);
    });

    view! {
        <div>
            {children()}
        </div>
    }
} 