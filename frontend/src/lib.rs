pub mod features;
pub mod services;

use leptos::*;
use ui_components::{
    theme::BaseTheme,
    ThemeProvider,
};
use features::HelloFeature;

#[component]
pub fn App() -> impl IntoView {
    let (theme, set_theme) = create_signal(BaseTheme::default_light());

    view! {
        <ThemeProvider theme=theme set_theme=set_theme>
            <HelloFeature />
        </ThemeProvider>
    }
} 