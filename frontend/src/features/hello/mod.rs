use leptos::*;
use ui_components::Hello as UIHello;
use crate::services;

#[component]
pub fn HelloFeature() -> impl IntoView {
    // Start with initial content
    let (message, set_message) = create_signal("Hello, World!".to_string());

    // Optionally update it after fetch (if you still want to fetch from backend)
    spawn_local(async move {
        if let Ok(response) = services::get_hello_message().await {
            // Use get_untracked since we don't need reactivity here
            if response.message != message.get_untracked() {
                set_message.set(response.message);
            }
        }
    });

    view! {
        <UIHello message=message />
    }
} 