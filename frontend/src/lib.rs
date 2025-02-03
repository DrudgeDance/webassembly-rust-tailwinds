pub mod features;
pub mod services;

use leptos::*;
use features::HelloFeature;
use ui_components::{
    theme::{Theme, ThemeProvider, SPRING_LIGHT_THEME, get_theme_variant, get_system_mode},
    ThemeSelector,
};

#[component]
pub fn App() -> impl IntoView {
    // Initialize theme based on system preference
    let initial_theme = {
        let system_mode = get_system_mode();
        get_theme_variant(&*SPRING_LIGHT_THEME, system_mode)
    };

    let (theme, set_theme) = create_signal(initial_theme);
    let (is_loading, set_is_loading) = create_signal(true);
    
    let handle_theme_switch = move |new_theme: Theme| {
        set_theme.set(new_theme);
    };

    // Hide loading state after initial render
    request_animation_frame(move || {
        set_is_loading.set(false);
    });

    view! {
        <ThemeProvider theme=theme>
            <div class="min-h-screen bg-background transition-colors duration-200">
                {move || {
                    if is_loading.get() {
                        view! {
                            <div class="fixed inset-0 bg-background flex items-center justify-center z-[100]">
                                <div class="text-text text-lg">Loading...</div>
                            </div>
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }
                }}
                <div class=move || format!("transition-opacity duration-75 {}", 
                    if is_loading.get() { "opacity-0" } else { "opacity-100" })>
                    <nav class="fixed top-0 left-0 right-0 p-4 bg-surface/80 backdrop-blur-sm shadow-sm z-40">
                        <div class="max-w-7xl mx-auto flex justify-between items-center">
                            <h1 class="text-2xl font-bold text-text">My App</h1>
                            <ThemeSelector
                                current_theme=theme
                                on_switch=handle_theme_switch
                            />
                        </div>
                    </nav>
                    <main class="pt-20">
                        <HelloFeature/>
                    </main>
                </div>
            </div>
        </ThemeProvider>
    }
} 