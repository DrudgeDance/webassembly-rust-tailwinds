use leptos::*;
use crate::theme::{BaseTheme, Mode};
use crate::components::layout::Layout;

#[component]
pub fn Hello(
    #[prop(into)] message: Signal<String>,
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    view! {
        <Layout theme=theme>
            <div class="text-center">
                <h2 class="text-4xl font-bold mb-4">
                    {move || message.get()}
                </h2>
                <p class="text-text-muted">
                    {move || format!("Current theme: {} - {}", 
                        if theme.get().mode == Mode::Light { "Light" } else { "Dark" },
                        theme.get().theme.map(|t| format!("{:?}", t)).unwrap_or_else(|| "Default".to_string())
                    )}
                </p>
            </div>
        </Layout>
    }
} 