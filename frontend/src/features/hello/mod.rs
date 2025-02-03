use leptos::*;
use ui_components::{
    components::hello::Hello as UIHello,
    theme::{Theme, Mode, Season},
};
use crate::services;

#[component]
pub fn HelloFeature() -> impl IntoView {
    let (message, set_message) = create_signal("Hello, World!".to_string());
    let theme = use_context::<Signal<Theme>>().expect("Theme should be provided by ThemeProvider");
    let set_theme = use_context::<WriteSignal<Theme>>().expect("Theme setter should be provided");
    
    let mode = create_memo(move |_| theme.get().mode);
    let season = create_memo(move |_| theme.get().season);
    
    let handle_theme_change = move |(new_mode, new_season): (Mode, Option<Season>)| {
        let new_theme = Theme {
            mode: new_mode,
            season: new_season,
            ..theme.get()
        };
        set_theme.set(new_theme);
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
            mode=mode
            season=season
            theme=theme
            on_theme_change=handle_theme_change
        />
    }
} 