use leptos::*;
use crate::theme::BaseTheme;
use crate::components::theme_selector02::ThemeSelector02;
use crate::layouts::footbar::Footbar;
use super::theme_switcher::create_theme_memo;

#[component]
pub fn Standard(
    #[prop(into)] theme: Signal<BaseTheme>,
    #[prop(optional)] header_title: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let theme_memo = create_theme_memo(theme);
    
    let set_theme = use_context::<WriteSignal<BaseTheme>>()
        .expect("ThemeProvider should be present");

    let on_theme_change = move |new_theme| set_theme.set(new_theme);
    
    view! {
        <div 
            class="min-h-screen flex flex-col w-full transition-colors duration-200"
            style=move || {
                let theme = theme_memo.get();
                format!(
                    "background-color: {}; color: {}",
                    theme.colors.background,
                    theme.colors.text,
                )
            }
        >
            // Header
            <header class="w-full">
                <nav 
                    class="flex items-center justify-between p-4 transition-colors duration-200"
                    style=move || {
                        let theme = theme_memo.get();
                        format!(
                            "background-color: {}; border-bottom: 1px solid {}",
                            theme.colors.surface,
                            theme.colors.primary,
                        )
                    }
                >
                    <div class="flex items-center space-x-4">
                        <h1 class="text-xl font-bold">
                            {header_title.unwrap_or("My App")}
                        </h1>
                    </div>
                    <div class="flex items-center space-x-4">
                        <ThemeSelector02 
                            theme=theme
                            on_theme_change=on_theme_change
                        />
                    </div>
                </nav>
            </header>

            // Main content
            <main 
                class="flex-grow flex flex-col w-full transition-colors duration-200"
                style=move || {
                    let theme = theme_memo.get();
                    format!(
                        "background-color: {}",
                        theme.colors.background,
                    )
                }
            >
                {children()}
            </main>

            <Footbar theme=theme />
        </div>
    }
} 