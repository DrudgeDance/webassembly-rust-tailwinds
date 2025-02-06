use leptos::*;
use crate::theme::BaseTheme;
use super::theme_switcher::create_theme_memo;
use crate::layouts::Base;

const DEFAULT_MESSAGE: &str = "Hello, World!";

#[component]
pub fn Hello(
    #[prop(into)] theme: Signal<BaseTheme>,
    #[prop(into, optional)] message: Option<Signal<String>>,
) -> impl IntoView {
    let theme_memo = create_theme_memo(theme);

    let display_message = create_memo(move |_| {
        message.map(|m| m.get()).unwrap_or_else(|| DEFAULT_MESSAGE.to_string())
    });

    let base_classes = "p-6 rounded-lg \
                       transition-colors duration-200 \
                       shadow-lg border";

    let theme_classes = move || {
        let colors = &theme_memo.get().colors;
        format!(
            "bg-{} text-{} border-{} shadow-{}/50 \
             hover:bg-{} hover:border-{} \
             selection:bg-{} selection:text-{}",
            colors.surface,
            colors.text,
            colors.border,
            colors.shadow_color,
            colors.hover_background,
            colors.hover_border,
            colors.selection_background,
            colors.selection_text
        )
    };

    view! {
        <Base theme=theme>
            <div class="flex items-center justify-center">
                <div class={move || format!("{} {}", base_classes, theme_classes())}>
                    <h1 class="text-2xl font-bold selection:text-inherit selection:bg-inherit">
                        {display_message}
                    </h1>
                    <p class=move || format!("mt-2 text-{} selection:text-inherit selection:bg-inherit", 
                        theme_memo.get().colors.text_muted)>
                        "Welcome to our themed application!"
                    </p>
                </div>
            </div>
        </Base>
    }
} 