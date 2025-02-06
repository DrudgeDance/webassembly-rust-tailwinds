use leptos::*;
use ui_components::{
    components::Login,
    theme::BaseTheme,
};

#[component]
pub fn LoginPage(
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    view! {
        <Login theme=theme />
    }
} 