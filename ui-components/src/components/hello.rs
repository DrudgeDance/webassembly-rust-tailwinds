use leptos::*;
use crate::styles::{theme::DEFAULT_THEME, constants::*};

#[component]
pub fn Hello(
    #[prop(into)] message: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="hello-container">
            <h1>{move || message.get()}</h1>
            <style>
                {format!(r#"
                    .hello-container {{
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        height: 100vh;
                        font-family: {FONT_FAMILY};
                        background-color: {background};
                    }}
                    h1 {{
                        color: {text};
                        padding: 20px;
                        border-radius: {BORDER_RADIUS};
                        background-color: white;
                        box-shadow: {BOX_SHADOW};
                    }}
                "#, 
                background = DEFAULT_THEME.background,
                text = DEFAULT_THEME.text,
                )}
            </style>
        </div>
    }
} 