use leptos::*;
use super::styles::SUBMIT_BUTTON_STYLES;

#[component]
pub fn SubmitButton(
    #[prop(into)] loading: Signal<bool>,
) -> impl IntoView {
    view! {
        <button
            type="submit"
            class=SUBMIT_BUTTON_STYLES
            style="background-color: var(--primary-color); color: white; focus-ring-color: var(--primary-color);"
            prop:disabled=loading
        >
            {move || if loading.get() {
                "Signing in..."
            } else {
                "Sign in"
            }}
        </button>
    }
} 