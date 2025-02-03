use leptos::*;

#[component]
pub fn Hello(
    #[prop(into)] message: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="flex justify-center items-center h-screen bg-gradient-to-br from-gray-50 to-gray-100">
            <h1 class="text-3xl font-bold text-gray-800 p-8 bg-white rounded-xl shadow-lg transform hover:scale-105 transition-transform duration-200">
                {move || message.get()}
            </h1>
        </div>
    }
} 