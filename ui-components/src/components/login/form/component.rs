use leptos::*;
use crate::theme::BaseTheme;
use super::{LoginInput, PasswordInput, SubmitButton, ErrorMessage};

#[component]
pub fn LoginForm(
    #[prop(into)] on_submit: Callback<(String, String)>,
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (show_password, set_show_password) = create_signal(false);
    let (error, set_error) = create_signal(Option::<String>::None);
    let (loading, set_loading) = create_signal(false);

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);
        set_error.set(None);

        on_submit.call((email.get(), password.get()));

        set_timeout(move || {
            set_loading.set(false);
            set_error.set(Some("Invalid email or password".to_string()));
        }, std::time::Duration::from_millis(1000));
    };

    view! {
        <form 
            on:submit=handle_submit 
            class="p-4 rounded-xl shadow-lg space-y-4 min-w-[300px] w-[400px]"
            style="background-color: var(--surface-color);"
        >
            <LoginInput
                value=email
                on_input=move |val| set_email.set(val)
                theme=theme
            />

            <PasswordInput
                value=password
                on_input=move |val| set_password.set(val)
                show_password=show_password
                on_toggle_visibility=move |_| set_show_password.update(|v| *v = !*v)
                theme=theme
            />

            <div class="flex items-center justify-between">
                <div class="flex items-center">
                    <input
                        type="checkbox"
                        id="remember"
                        class="h-4 w-4"
                        style="accent-color: var(--primary-color);"
                    />
                    <label 
                        for="remember" 
                        class="ml-2 text-sm"
                    >
                        "Remember me"
                    </label>
                </div>
                <a 
                    href="#" 
                    class="text-sm hover:underline"
                    style="color: var(--primary-color);"
                >
                    "Forgot password?"
                </a>
            </div>

            <ErrorMessage error=error />
            <SubmitButton loading=loading />

            <div class="mt-6 text-center">
                <span class="text-sm">"Don't have an account? "</span>
                <a 
                    href="#" 
                    class="text-sm hover:underline"
                    style="color: var(--primary-color);"
                >
                    "Sign up"
                </a>
            </div>
        </form>
    }
} 