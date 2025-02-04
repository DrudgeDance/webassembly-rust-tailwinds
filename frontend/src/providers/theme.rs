use leptos::*;
use ui_components::theme::BaseTheme;

#[component]
pub fn ThemeProvider(
    #[prop(into)] theme: Signal<BaseTheme>,
    #[prop(into)] set_theme: WriteSignal<BaseTheme>,
    children: Children,
) -> impl IntoView {
    provide_context(theme);
    provide_context(set_theme);

    view! {
        <div class="h-full">
            {children()}
        </div>
    }
} 