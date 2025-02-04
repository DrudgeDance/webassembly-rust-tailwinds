use leptos::*;
use crate::theme::BaseTheme;
use crate::layouts::navbar::Navbar;
use super::theme_switcher::create_theme_memo;

#[component]
pub fn Base(
    #[prop(into)] theme: Signal<BaseTheme>,
    children: Children,
) -> impl IntoView {
    let theme_memo = create_theme_memo(theme);

    view! {
        <div 
            class="min-h-screen transition-colors duration-200"
            style=move || {
                let theme = theme_memo.get();
                format!(
                    "background-color: {}; color: {}",
                    theme.colors.background,
                    theme.colors.text,
                )
            }
        >
            <Navbar theme=theme/>
            <main 
                class="flex-grow flex flex-col items-center justify-center space-y-8 p-8 transition-colors duration-200"
                style=move || {
                    let theme = theme_memo.get();
                    format!(
                        "background-color: {}; border-top: 1px solid {}; box-shadow: 0 -4px 6px {}",
                        theme.colors.surface,
                        theme.colors.border,
                        theme.colors.shadow,
                    )
                }
            >
                {children()}
            </main>
        </div>
    }
} 