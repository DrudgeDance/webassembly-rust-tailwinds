use leptos::*;
use crate::theme::BaseTheme;
use super::theme_switcher::create_theme_memo;

#[component]
pub fn BaseInput(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_input: Callback<String>,
    input_type: Box<dyn Fn() -> String>,
    #[prop(into)] placeholder: String,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    let theme_memo = create_theme_memo(theme);
    let class = class.unwrap_or_default();
    
    let base_classes = "w-full px-3 py-2 rounded-lg \
                       border-2 focus:outline-none \
                       transition-colors duration-200";

    let theme_classes = move || {
        let theme = theme_memo.get();
        match theme.theme {
            Some(crate::theme::Theme::Spring) => {
                match theme.mode {
                    crate::theme::Mode::Light => "bg-green-50 text-green-900 \
                        placeholder-green-500 border-green-500/50 \
                        focus:border-green-500 focus:ring-2 focus:ring-green-500/50 \
                        hover:bg-green-100 hover:border-green-400 \
                        disabled:bg-green-50 disabled:text-green-300 disabled:border-green-200 \
                        selection:bg-green-200 selection:text-green-900",
                    crate::theme::Mode::Dark => "bg-green-900 text-green-100 \
                        placeholder-green-400 border-green-400/50 \
                        focus:border-green-400 focus:ring-2 focus:ring-green-400/50 \
                        hover:bg-green-800 hover:border-green-500 \
                        disabled:bg-green-800 disabled:text-green-600 disabled:border-green-700 \
                        selection:bg-green-700 selection:text-green-100",
                }
            },
            Some(crate::theme::Theme::Summer) => {
                match theme.mode {
                    crate::theme::Mode::Light => "bg-orange-50 text-orange-900 \
                        placeholder-orange-500 border-orange-500/50 \
                        focus:border-orange-500 focus:ring-2 focus:ring-orange-500/50 \
                        hover:bg-orange-100 hover:border-orange-400 \
                        disabled:bg-orange-50 disabled:text-orange-300 disabled:border-orange-200 \
                        selection:bg-orange-200 selection:text-orange-900",
                    crate::theme::Mode::Dark => "bg-orange-900 text-orange-100 \
                        placeholder-orange-400 border-orange-400/50 \
                        focus:border-orange-400 focus:ring-2 focus:ring-orange-400/50 \
                        hover:bg-orange-800 hover:border-orange-500 \
                        disabled:bg-orange-800 disabled:text-orange-600 disabled:border-orange-700 \
                        selection:bg-orange-700 selection:text-orange-100",
                }
            },
            None => {
                match theme.mode {
                    crate::theme::Mode::Light => "bg-white text-gray-900 \
                        placeholder-gray-500 border-blue-500/50 \
                        focus:border-blue-500 focus:ring-2 focus:ring-blue-500/50 \
                        hover:bg-gray-50 hover:border-blue-400 \
                        disabled:bg-gray-50 disabled:text-gray-300 disabled:border-gray-200 \
                        selection:bg-blue-100 selection:text-blue-900",
                    crate::theme::Mode::Dark => "bg-gray-800 text-gray-100 \
                        placeholder-gray-400 border-blue-400/50 \
                        focus:border-blue-400 focus:ring-2 focus:ring-blue-400/50 \
                        hover:bg-gray-700 hover:border-blue-500 \
                        disabled:bg-gray-700 disabled:text-gray-600 disabled:border-gray-700 \
                        selection:bg-blue-900 selection:text-blue-100",
                }
            }
        }
    };
    
    view! {
        <div class="relative">
            <input
                type=input_type
                placeholder=placeholder
                value=value
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    on_input.call(val);
                }
                class={move || format!("{} {} {}", base_classes, theme_classes(), class)}
            />
            {children.map(|c| view! { <>{c()}</> })}
        </div>
    }
} 