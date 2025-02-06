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
        <div class="min-h-screen flex items-center justify-center">
            <LoginMini theme=theme />
        </div>
    }
} 