pub mod features;
pub mod services;

use leptos::*;
use features::HelloFeature;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <HelloFeature />
    }
} 