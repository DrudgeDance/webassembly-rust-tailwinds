use leptos::*;
use ui_components::Hello as UIHello;
use crate::services;

#[component]
pub fn HelloFeature() -> impl IntoView {
    let (message, set_message) = create_signal(String::from("Loading..."));

    create_effect(move |_| {
        spawn_local(async move {
            match services::get_hello_message().await {
                Ok(response) => set_message.set(response.message),
                Err(e) => set_message.set(format!("Error: {}", e)),
            }
        });
    });

    view! {
        <UIHello message=message />
    }
} 