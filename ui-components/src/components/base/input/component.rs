use leptos::*;
use crate::theme::BaseTheme;
use super::theme_switcher::create_theme_memo;

#[component]
pub fn BaseInput(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_input: Callback<String>,
    input_type: Box<dyn Fn() -> String>,
    #[prop(into)] placeholder: String,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    let theme_memo = create_theme_memo(theme);
    
    let input_style = move || {
        let colors = &theme_memo.get().colors;
        format!(
            "background-color: {}; \
             color: {}; \
             border: 1px solid {}; \
             &:focus {{ box-shadow: 0 0 0 2px {} }}; \
             &:hover {{ background-color: {} }}; \
             &:disabled {{ background-color: {}; color: {} }}; \
             &::placeholder {{ color: {} }};",
            colors.background,
            colors.text,
            colors.border,
            colors.focus_ring,
            colors.hover_background,
            colors.disabled_background,
            colors.disabled_text,
            colors.placeholder,
        )
    };
    
    view! {
        <div class="relative">
            <input
                type=input_type
                placeholder=placeholder
                value=value
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    on_input.call(val);
                }
                class=class
                style=input_style
            />
            {children.map(|c| view! { <>{c()}</> })}
        </div>
    }
} 