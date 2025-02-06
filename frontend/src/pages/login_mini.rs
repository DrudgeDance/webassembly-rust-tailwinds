use leptos::*;
use ui_components::{
    components::LoginMini,
    theme::BaseTheme,
};

#[component]
pub fn LoginMiniPage(
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    view! {
        <LoginMini theme=theme />
    }
} 