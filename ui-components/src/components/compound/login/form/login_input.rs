use leptos::*;
use crate::{
    components::base::input::BaseInput,
    theme::BaseTheme,
};
use super::styles::INPUT_BASE_STYLES;

#[component]
pub fn LoginInput(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_input: Callback<String>,
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    let input_type: Box<dyn Fn() -> String> = Box::new(|| "email".to_string());

    view! {
        <BaseInput
            value=value
            on_input=on_input
            input_type=input_type
            placeholder="Enter your email".to_string()
            class=INPUT_BASE_STYLES.to_string()
            theme=theme
        />
    }
} 