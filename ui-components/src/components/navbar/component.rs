use leptos::*;
use crate::theme::BaseTheme;
use crate::components::theme_selector::ThemeSelector;
use crate::defaults::APP_TITLE;

#[component]
pub fn Navbar(
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    let theme_memo = create_memo(move |_| theme.get());

    view! {
        <nav 
            class="p-4 flex justify-between items-center"
            style=move || format!(
                "background-color: {}; color: {}; border-bottom: 1px solid {}", 
                theme_memo.get().colors.background,
                theme_memo.get().colors.text,
                theme_memo.get().colors.text_muted
            )
        >
            <h1 class="text-2xl font-bold">{APP_TITLE}</h1>
            <ThemeSelector theme=theme />
        </nav>
    }
} 