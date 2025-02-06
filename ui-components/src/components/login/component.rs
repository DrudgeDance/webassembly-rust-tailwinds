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
            <div class="w-full flex-grow flex flex-col items-center justify-center px-4 sm:px-6 lg:px-8">
                <div class="w-full max-w-sm sm:max-w-md">
                    <LoginForm 
                        on_submit=handle_submit 
                        theme=theme
                    />
                </div>
            </div>
        </Standard>
    }
} 