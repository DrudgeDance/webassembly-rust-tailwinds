use leptos::*;
use crate::theme::BaseTheme;
use super::{LoginMiniInput, PasswordMiniInput};

#[component]
pub fn LoginMiniForm(
    #[prop(into)] on_submit: Callback<(String, String)>,
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    let (login, set_login) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (show_password, set_show_password) = create_signal(false);

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        on_submit.call((login.get(), password.get()));
    };

    view! {
        <form 
            on:submit=handle_submit 
            class="p-2 rounded-xl shadow-lg space-y-2 min-w-[200px] w-[250px]"
            style="background-color: var(--surface-color);"
        >
            <LoginMiniInput
                value=login
                on_input=move |val| set_login.set(val)
                theme=theme
            />

            <PasswordMiniInput
                value=password
                on_input=move |val| set_password.set(val)
                show_password=show_password
                on_toggle_visibility=move |_| set_show_password.update(|v| *v = !*v)
                theme=theme
            />
        </form>
    }
} 