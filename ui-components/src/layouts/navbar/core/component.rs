use leptos::*;
use crate::theme::BaseTheme;
use crate::components::compound::theme_selector02::ThemeSelector02;
use super::theme_switcher::create_theme_memo;

#[component]
pub fn Navbar(
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    let theme_memo = create_theme_memo(theme);
    
    let set_theme = use_context::<WriteSignal<BaseTheme>>()
        .expect("ThemeProvider should be present");

    let on_theme_change = move |new_theme| set_theme.set(new_theme);

    view! {
        <nav 
            class="flex items-center justify-between p-4 transition-colors duration-200"
            style=move || {
                let theme = theme_memo.get();
                format!(
                    "background-color: {}; color: {}; border-bottom: 1px solid {}",
                    theme.colors.background,
                    theme.colors.text,
                    theme.colors.border,
                )
            }
        >
            <div class="flex items-center space-x-4">
                <h1 class="text-xl font-bold">My App</h1>
            </div>
            <div class="flex items-center space-x-4">
                <ThemeSelector02 
                    theme=theme
                    on_theme_change=on_theme_change
                />
            </div>
        </nav>
    }
} 