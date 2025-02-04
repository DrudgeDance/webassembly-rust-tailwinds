use leptos::*;
use leptos::logging::log;
use crate::theme::BaseTheme;

#[component]
pub fn ThemeProvider(
    #[prop(into)] theme: Signal<BaseTheme>,
    #[prop(into)] set_theme: WriteSignal<BaseTheme>,
    children: Children,
) -> impl IntoView {
    // Provide theme context to children
    provide_context(theme);
    provide_context(set_theme);

    create_effect(move |_| {
        log!("Theme mode: {:?}, theme: {:?}", theme.get().mode, theme.get().theme);
    });

    view! {
        <div>
            {children()}
        </div>
    }
} 