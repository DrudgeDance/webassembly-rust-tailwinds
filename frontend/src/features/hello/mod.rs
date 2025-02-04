use leptos::*;
use ui_components::{
    components::hello::Hello as UIHello,
    theme::{BaseTheme, Mode, Theme},
};
use crate::services;

#[component]
pub fn HelloFeature() -> impl IntoView {
    let (message, set_message) = create_signal("Hello, World!".to_string());
    let base_theme = use_context::<Signal<BaseTheme>>().expect("Theme should be provided by ThemeProvider");
    let set_theme = use_context::<WriteSignal<BaseTheme>>().expect("Theme setter should be provided");
    
    let theme = create_memo(move |_| base_theme.get());
    
    let handle_theme_change = move |(new_mode, new_theme): (Mode, Option<Theme>)| {
        let current = base_theme.get();
        let new_base_theme = BaseTheme {
            mode: new_mode,
            theme: new_theme,
            ..current
        };
        set_theme.set(new_base_theme);
    };

    // Optionally update it after fetch
    spawn_local(async move {
        if let Ok(response) = services::get_hello_message().await {
            if response.message != message.get_untracked() {
                set_message.set(response.message);
            }
        }
    });

    view! {
        <UIHello 
            message=message
            theme=theme
            on_theme_change=handle_theme_change
        />
    }
} 