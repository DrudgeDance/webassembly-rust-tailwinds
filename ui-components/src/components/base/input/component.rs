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
    let class = class.unwrap_or_default();
    
    let base_classes = "w-full px-3 py-2 rounded-lg \
                       border-2 focus:outline-none \
                       transition-colors duration-200";

    let theme_classes = move || {
        let colors = &theme_memo.get().colors;
        format!(
            "bg-{} text-{} \
             placeholder-{} border-{} \
             focus:border-{} focus:ring-2 focus:ring-{} \
             hover:bg-{} hover:border-{} \
             disabled:bg-{} disabled:text-{} disabled:border-{} \
             selection:bg-{} selection:text-{}",
            colors.background,
            colors.text,
            colors.placeholder,
            colors.border,
            colors.focus_border,
            colors.focus_ring,
            colors.hover_background,
            colors.hover_border,
            colors.disabled_background,
            colors.disabled_text,
            colors.disabled_border,
            colors.selection_background,
            colors.selection_text,
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
                class={move || format!("{} {} {}", base_classes, theme_classes(), class)}
            />
            {children.map(|c| view! { <>{c()}</> })}
        </div>
    }
} 