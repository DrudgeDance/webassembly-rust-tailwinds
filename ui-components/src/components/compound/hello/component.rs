use leptos::*;
use crate::theme::BaseTheme;
use super::theme_switcher::create_theme_memo;
use crate::layouts::Standard;

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

    // Static base classes that are common across all themes
    let base_classes = "p-6 rounded-lg transition-all duration-200 shadow-lg focus:outline-none cursor-pointer border-2 border-solid";

    let theme_classes = move || {
        let colors = &theme_memo.get().colors;
        format!(
            "bg-{} text-{} border-{} \
            shadow-{}/50 \
            hover:bg-{} hover:border-{} \
            focus:border-{} focus:ring-2 focus:ring-{} \
            active:bg-{} active:border-{} \
            selection:bg-{} selection:text-{}",
            colors.background,
            colors.text,
            colors.border,
            colors.shadow_color,
            colors.hover_background,
            colors.hover_border,
            colors.focus_border,
            colors.focus_ring,
            colors.active_background,
            colors.active_border,
            colors.selection_background,
            colors.selection_text,
        )
    };

    let heading_classes = move || {
        let colors = &theme_memo.get().colors;
        format!("text-2xl font-bold text-{}", colors.heading_text)
    };

    let text_muted_classes = move || {
        let colors = &theme_memo.get().colors;
        format!("mt-2 text-{}", colors.text_muted)
    };

    view! {
        <Standard theme=theme header_title="Hello">
            <div class="flex-grow flex flex-col items-center justify-start pt-8">
                <div 
                    class={move || format!("{} {}", base_classes, theme_classes())}
                    tabindex="0"
                >
                    <h1 class=heading_classes>
                        {display_message}
                    </h1>
                    <p class=text_muted_classes>
                        "Welcome to our themed application!"
                    </p>
                </div>
            </div>
        </Standard>
    }
}