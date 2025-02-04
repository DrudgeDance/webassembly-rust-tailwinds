use leptos::*;
use ui_components::components::Hello as UIHello;
use ui_components::theme::BaseTheme;
use crate::services;

#[component]
pub fn HelloFeature(
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    let (message_signal, set_message) = create_signal("Loading...".to_string());

    let _message_resource = create_local_resource(
        || (),
        move |_| async move {
            if let Ok(response) = services::get_hello_message().await {
                set_message.set(response.message);
            }
        }
    );

    view! {
        <UIHello 
            message=message_signal
            theme=theme
        />
    }
} 