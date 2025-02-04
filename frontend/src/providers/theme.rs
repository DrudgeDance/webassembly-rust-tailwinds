use leptos::*;
use ui_components::theme::{BaseTheme, Mode, Theme};

#[component]
pub fn ThemeProvider(
    #[prop(into)] theme: Signal<BaseTheme>,
    #[prop(into)] set_theme: WriteSignal<BaseTheme>,
    children: Children,
) -> impl IntoView {
    provide_context(theme);
    provide_context(set_theme);

    // Update data-theme attribute when theme changes
    create_effect(move |_| {
        let current_theme = theme.get();
        let theme_name = match (current_theme.mode, current_theme.theme) {
            (Mode::Light, None) => "light",
            (Mode::Dark, None) => "dark",
            (Mode::Light, Some(Theme::Spring)) => "light-spring",
            (Mode::Dark, Some(Theme::Spring)) => "dark-spring",
            (Mode::Light, Some(Theme::Summer)) => "light-summer",
            (Mode::Dark, Some(Theme::Summer)) => "dark-summer",
        };
        
        // Update document theme
        let document = web_sys::window()
            .unwrap()
            .document()
            .unwrap();
        document
            .document_element()
            .unwrap()
            .set_attribute("data-theme", theme_name)
            .unwrap();
    });

    view! {
        <div class="h-full">
            {children()}
        </div>
    }
}

pub fn use_theme() -> (Signal<BaseTheme>, WriteSignal<BaseTheme>) {
    (
        use_context::<Signal<BaseTheme>>().expect("ThemeProvider not found"),
        use_context::<WriteSignal<BaseTheme>>().expect("ThemeProvider not found"),
    )
} 