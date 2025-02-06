use leptos::*;
use crate::{
    components::base::input::BaseInput,
    theme::BaseTheme,
};
use super::styles::{INPUT_WITH_BUTTON_STYLES, BUTTON_STYLES};

#[component]
pub fn PasswordInput(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_input: Callback<String>,
    #[prop(into)] show_password: Signal<bool>,
    #[prop(into)] on_toggle_visibility: Callback<()>,
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    let input_type: Box<dyn Fn() -> String> = Box::new(move || {
        if show_password.get() {
            "text".to_string()
        } else {
            "password".to_string()
        }
    });

    let toggle_button = move || {
        view! {
            <button
                type="button"
                on:click=move |_| on_toggle_visibility.call(())
                class=BUTTON_STYLES
            >
                {move || if show_password.get() {
                    view! {
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3.98 8.223A10.477 10.477 0 001.934 12C3.226 16.338 7.244 19.5 12 19.5c.993 0 1.953-.138 2.863-.395M6.228 6.228A10.45 10.45 0 0112 4.5c4.756 0 8.773 3.162 10.065 7.498a10.523 10.523 0 01-4.293 5.774M6.228 6.228L3 3m3.228 3.228l3.65 3.65m7.894 7.894L21 21m-3.228-3.228l-3.65-3.65m0 0a3 3 0 10-4.243-4.243m4.242 4.242L9.88 9.88" />
                        </svg>
                    }
                } else {
                    view! {
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M2.036 12.322a1.012 1.012 0 010-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178z" />
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                        </svg>
                    }
                }}
            </button>
        }
    };

    view! {
        <BaseInput
            value=value
            on_input=on_input
            input_type=input_type
            placeholder="Enter your password".to_string()
            class=INPUT_WITH_BUTTON_STYLES.to_string()
            theme=theme
        >
            {toggle_button}
        </BaseInput>
    }
} 