use leptos::*;
use crate::{
    layouts::Standard,
    theme::BaseTheme,
};
use super::LoginMiniForm;

#[component]
pub fn LoginMini(
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    let handle_submit = move |(email, _password): (String, String)| {
        let _ = web_sys::console::log_1(&format!("Login attempt with email: {}", email).into());
    };

    view! {
        <Standard theme=theme header_title="Quick Login">
            <div class="flex-grow flex flex-col items-center justify-start">
                <div class="w-full">
                    <LoginMiniForm
                        on_submit=handle_submit
                        theme=theme
                    />
                </div>
            </div>
        </Standard>
    }
} 