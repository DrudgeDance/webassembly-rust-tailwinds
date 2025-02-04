use leptos::*;
use crate::theme::BaseTheme;
use super::theme_switcher::create_theme_memo;

#[component]
pub fn MainContent(
    #[prop(into)] theme: Signal<BaseTheme>,
    children: Children,
) -> impl IntoView {
    let theme_memo = create_theme_memo(theme);

    view! {
        <main 
            class="flex-grow flex flex-col items-center justify-center p-8"
            style=move || {
                let theme = theme_memo.get();
                format!(
                    "background-color: {}; color: {}; border-top: 1px solid {}; box-shadow: inset 0 2px 4px {}",
                    theme.colors.background,
                    theme.colors.text,
                    theme.colors.border,
                    theme.colors.shadow,
                )
            }
        >
            {children()}
        </main>
    }
} 