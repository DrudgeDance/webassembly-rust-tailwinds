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
            class=move || {
                let theme = theme_memo.get();
                format!(
                    "min-h-screen transition-colors duration-200 bg-{} text-{}",
                    theme.colors.background,
                    theme.colors.text
                )
            }
        >
            <Navbar theme=theme/>
            <main 
                class=move || {
                    let theme = theme_memo.get();
                    format!(
                        "flex-grow flex flex-col items-center justify-center space-y-8 p-8 \
                         transition-colors duration-200 bg-{}",
                        theme.colors.background
                    )
                }
            >
                {children()}
            </main>
        </div>
    }
} 