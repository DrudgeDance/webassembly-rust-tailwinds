pub mod features;
pub mod services;

use leptos::*;
use crate::features::HelloFeature;
use ui_components::{
    theme::{Theme, ThemeProvider},
};

#[component]
pub fn App() -> impl IntoView {
    let (theme, set_theme) = create_signal(Theme::default_light());

    view! {
        <ThemeProvider theme=theme set_theme=set_theme>
            <HelloFeature />
        </ThemeProvider>
    }
} 