use leptos::*;
use crate::theme::BaseTheme;
use super::LoginMiniForm;

#[component]
pub fn LoginMini(
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    let handle_submit = move |(email, _password): (String, String)| {
        let _ = web_sys::console::log_1(&format!("Login attempt with email: {}", email).into());
    };

    view! {
        <div class="flex items-center justify-center min-h-screen">
            <LoginMiniForm
                on_submit=handle_submit
                theme=theme
            />
        </div>
    }
} 