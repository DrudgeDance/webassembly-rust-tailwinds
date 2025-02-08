use leptos::*;
use crate::theme::BaseTheme;
use super::theme_switcher::create_theme_memo;

#[component]
pub fn Footbar(
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    let theme_memo = create_theme_memo(theme);
    
    view! {
        <footer 
            class="w-full p-4 text-center transition-colors duration-200"
            style=move || {
                let theme = theme_memo.get();
                format!(
                    "background-color: {}; color: {}; border-top: 1px solid {}",
                    theme.colors.surface,
                    theme.colors.text_muted,
                    theme.colors.border,
                )
            }
        >
            <p>"© 2024 My App. All rights reserved."</p>
        </footer>
    }
} 