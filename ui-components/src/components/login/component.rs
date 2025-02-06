use leptos::*;
use crate::{
    layouts::Standard,
    theme::BaseTheme,
};
use super::LoginForm;

#[component]
pub fn Login(
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    let handle_submit = move |(email, _password): (String, String)| {
        // Use web_sys::console for logging in web context
        let _ = web_sys::console::log_1(&format!("Login attempt with email: {}", email).into());
        // TODO: Implement actual login logic here
    };

    view! {
        <Standard theme=theme header_title="Login">
            <div class="w-full max-w-md mx-auto mt-10">
                <div 
                    class="shadow-md rounded-lg px-8 py-6 mb-4"
                    style="background-color: var(--surface-color); border: 1px solid var(--border-color);"
                >
                    <LoginForm 
                        on_submit=handle_submit 
                        theme=theme
                    />
                </div>
            </div>
        </Standard>
    }
} 