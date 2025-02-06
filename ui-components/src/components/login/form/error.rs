use leptos::*;

#[component]
pub fn ErrorMessage(
    #[prop(into)] error: Signal<Option<String>>,
) -> impl IntoView {
    view! {
        {move || error.get().map(|err| view! {
            <div 
                class="text-sm mt-2"
                style="color: var(--error-color);"
            >
                {err}
            </div>
        })}
    }
} 