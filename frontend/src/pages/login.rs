use leptos::*;
use ui_components::{
    layouts::Standard,
    theme::BaseTheme,
};

#[component]
pub fn LoginPage(
    #[prop(into)] theme: Signal<BaseTheme>,
) -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (show_password, set_show_password) = create_signal(false);
    let (error, set_error) = create_signal(Option::<String>::None);
    let (loading, set_loading) = create_signal(false);

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);
        set_error.set(None);

        // TODO: Implement actual login logic here
        // For now, just simulate a delay and show an error
        set_timeout(move || {
            set_loading.set(false);
            set_error.set(Some("Invalid email or password".to_string()));
        }, std::time::Duration::from_millis(1000));
    };

    view! {
        <Standard theme=theme header_title="Login">
            <div class="w-full max-w-md mx-auto mt-10">
                <div 
                    class="shadow-md rounded-lg px-8 py-6 mb-4"
                    style="background-color: var(--surface-color); border: 1px solid var(--border-color);"
                >
                    <h2 class="text-2xl font-bold mb-6 text-center">"Welcome Back"</h2>
                    
                    <form on:submit=handle_submit class="space-y-6">
                        // Email field
                        <div>
                            <label 
                                for="email" 
                                class="block text-sm font-medium mb-2"
                            >
                                "Email"
                            </label>
                            <input
                                type="email"
                                id="email"
                                required
                                class="w-full px-3 py-2 rounded-md focus:outline-none focus:ring-2"
                                style="background-color: var(--background-color); color: var(--text-color); border: 1px solid var(--border-color); focus-ring-color: var(--primary-color);"
                                placeholder="Enter your email"
                                prop:value=move || email.get()
                                on:input=move |ev| {
                                    set_email.set(event_target_value(&ev));
                                }
                            />
                        </div>

                        // Password field
                        <div>
                            <label 
                                for="password" 
                                class="block text-sm font-medium mb-2"
                            >
                                "Password"
                            </label>
                            <div class="relative">
                                <input
                                    type=move || if show_password.get() { "text" } else { "password" }
                                    id="password"
                                    required
                                    class="w-full px-3 py-2 rounded-md focus:outline-none focus:ring-2"
                                    style="background-color: var(--background-color); color: var(--text-color); border: 1px solid var(--border-color); focus-ring-color: var(--primary-color);"
                                    placeholder="Enter your password"
                                    prop:value=move || password.get()
                                    on:input=move |ev| {
                                        set_password.set(event_target_value(&ev));
                                    }
                                />
                                <button
                                    type="button"
                                    class="absolute inset-y-0 right-0 pr-3 flex items-center"
                                    on:click=move |_| set_show_password.update(|v| *v = !*v)
                                >
                                    {move || if show_password.get() {
                                        "🙈"
                                    } else {
                                        "👁️"
                                    }}
                                </button>
                            </div>
                        </div>

                        // Remember me and Forgot password
                        <div class="flex items-center justify-between">
                            <div class="flex items-center">
                                <input
                                    type="checkbox"
                                    id="remember"
                                    class="h-4 w-4"
                                    style="accent-color: var(--primary-color);"
                                />
                                <label 
                                    for="remember" 
                                    class="ml-2 text-sm"
                                >
                                    "Remember me"
                                </label>
                            </div>
                            <a 
                                href="#" 
                                class="text-sm hover:underline"
                                style="color: var(--primary-color);"
                            >
                                "Forgot password?"
                            </a>
                        </div>

                        // Error message
                        {move || error.get().map(|err| view! {
                            <div 
                                class="text-sm mt-2"
                                style="color: var(--error-color);"
                            >
                                {err}
                            </div>
                        })}

                        // Submit button
                        <button
                            type="submit"
                            class="w-full py-2 px-4 rounded-md hover:opacity-90 focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 transition-colors duration-200"
                            style="background-color: var(--primary-color); color: white; focus-ring-color: var(--primary-color);"
                            prop:disabled=loading
                        >
                            {move || if loading.get() {
                                "Signing in..."
                            } else {
                                "Sign in"
                            }}
                        </button>
                    </form>

                    // Sign up link
                    <div class="mt-6 text-center">
                        <span class="text-sm">"Don't have an account? "</span>
                        <a 
                            href="#" 
                            class="text-sm hover:underline"
                            style="color: var(--primary-color);"
                        >
                            "Sign up"
                        </a>
                    </div>
                </div>
            </div>
        </Standard>
    }
} 