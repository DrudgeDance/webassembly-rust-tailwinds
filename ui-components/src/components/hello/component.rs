use leptos::*;

#[component]
pub fn Hello(
    #[prop(into)] message: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="flex justify-center items-center h-screen bg-background">
            <h1 class="text-3xl font-bold text-text p-8 bg-surface rounded-xl shadow-lg transform hover:scale-105 transition-transform duration-200">
                {move || message.get()}
            </h1>
        </div>
    }
} 