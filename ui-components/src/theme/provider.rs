use leptos::*;
use leptos::logging::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use super::Theme;

#[component]
pub fn ThemeProvider(
    #[prop(into)] theme: Signal<Theme>,
    children: Children,
) -> impl IntoView {
    create_effect(move |_| {
        let theme = theme.get();
        let colors = theme.colors;
        let root = document().document_element().unwrap();
        let root: HtmlElement = root.dyn_into().unwrap();
        let style = root.style();

        // Set CSS variables with proper color format
        let set_color = |name: &str, value: &str| {
            let color_value = if !value.starts_with('#') {
                format!("#{}", value)
            } else {
                value.to_string()
            };
            if let Err(e) = style.set_property(name, &color_value) {
                log!("Failed to set CSS variable {}: {:?}", name, e);
            }
        };

        // Set CSS variables
        set_color("--background", &colors.background);
        set_color("--surface", &colors.surface);
        set_color("--text", &colors.text);
        set_color("--text-muted", &colors.text_muted);
        set_color("--primary", &colors.primary);
        set_color("--secondary", &colors.secondary);
        set_color("--accent", &colors.accent);
        set_color("--seasonal-primary", &colors.seasonal_primary);
        set_color("--seasonal-secondary", &colors.seasonal_secondary);
        set_color("--seasonal-accent", &colors.seasonal_accent);

        log!("Theme updated to: {}", theme.name);
    });

    view! { <>{children()}</> }
} 