use leptos::*;
use crate::theme::BaseTheme;
use crate::components::navbar::Navbar;

#[component]
pub fn Layout(
    #[prop(into)] theme: Signal<BaseTheme>,
    children: Children,
) -> impl IntoView {
    let theme_memo = create_memo(move |_| theme.get());

    view! {
        <div 
            class="flex flex-col h-screen"
            style=move || format!(
                "background-color: {}; color: {}; box-shadow: 0 4px 6px {}", 
                theme_memo.get().colors.background,
                theme_memo.get().colors.text,
                theme_memo.get().colors.text_muted
            )
        >
            <Navbar theme=theme />
            <main class="flex-grow flex flex-col items-center justify-center space-y-8">
                {children()}
            </main>
        </div>
    }
} 