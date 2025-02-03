# UI Component Optimizations

## Synchronous Initial State Pattern

### Problem
When components fetch data asynchronously, they often start with an empty or loading state, causing visual flickering or loading indicators. This creates a suboptimal user experience, especially in WebAssembly applications where the initial render should be near-instant.

### Solution: Synchronous Initial State
Instead of starting with an empty state and waiting for async data, we provide an immediate synchronous state while fetching updates in the background.

```rust
// ❌ Without Synchronous Initial State
pub fn Component() -> impl IntoView {
    let (data, set_data) = create_signal(String::new()); // Empty initial state
    
    spawn_local(async move {
        if let Ok(response) = fetch_data().await {
            set_data.set(response);
        }
    });
    
    view! { <Display data=data /> } // Shows nothing initially
}

// ✅ With Synchronous Initial State
pub fn Component() -> impl IntoView {
    let (data, set_data) = create_signal("Initial Value".to_string()); // Immediate content
    
    spawn_local(async move {
        if let Ok(response) = fetch_data().await {
            if response != data.get() {  // Only update if different
                set_data.set(response);
            }
        }
    });
    
    view! { <Display data=data /> } // Shows content immediately
}
```

### Benefits
1. **Instant First Paint**: Users see content immediately without waiting for async operations
2. **No Loading Flicker**: Eliminates the need for loading states in many cases
3. **Progressive Enhancement**: Content updates seamlessly if backend data differs
4. **Better WebAssembly Performance**: Takes advantage of Wasm's fast initial render
5. **Improved Perceived Performance**: Application feels more responsive

### When to Use
- Components that fetch data but have a reasonable default state
- Static content that rarely changes
- User interfaces where immediate feedback is important
- WebAssembly applications where performance is critical

### When Not to Use
- When the initial state cannot be reasonably predicted
- When showing stale data could be misleading
- When the async data is critical for security or functionality

### Implementation Tips
1. Choose meaningful initial states that match expected data structure
2. Compare new data with current state to avoid unnecessary updates
3. Consider using this pattern with Suspense boundaries for larger applications
4. Document the expected data flow for maintainability

### Example in Practice
```rust
#[component]
pub fn HelloFeature() -> impl IntoView {
    // Immediate initial state
    let (message, set_message) = create_signal("Hello, World!".to_string());

    // Background update
    spawn_local(async move {
        if let Ok(response) = services::get_hello_message().await {
            if response.message != message.get() {
                set_message.set(response.message);
            }
        }
    });

    view! {
        <UIHello message=message />
    }
}
```

This pattern is particularly effective in Leptos applications using WebAssembly, where we can leverage the fast initial render capabilities of Wasm while still maintaining the ability to update content dynamically. 

## Loading State Patterns

### 2. Hybrid Loading Pattern

Sometimes we want to show a brief loading state for the entire application while still maintaining fast component-level rendering. Here's how to implement a fast, page-wide loading state that doesn't interfere with component performance:

```rust
#[component]
pub fn App() -> impl IntoView {
    let (is_loading, set_is_loading) = create_signal(true);
    
    // Hide loading state after initial render
    request_animation_frame(move || {
        set_is_loading.set(false);
    });

    view! {
        <div>
            // Loading overlay
            {move || if is_loading.get() {
                view! {
                    <div class="fixed inset-0 z-[100]">
                        <div>Loading...</div>
                    </div>
                }.into_view()
            } else {
                view! {}.into_view()
            }}

            // Main content (initially hidden)
            <div class=move || format!("transition-opacity {}", 
                if is_loading.get() { "opacity-0" } else { "opacity-100" })>
                <AppContent/>
            </div>
        </div>
    }
}
```

#### Key Features
1. **Single Frame Loading**: The loading state only shows for one animation frame
2. **Full Page Coverage**: Loading indicator covers all content
3. **Smooth Transition**: Content fades in when ready
4. **No Component Interference**: Components can still use Synchronous Initial State pattern

#### Best Practices
1. Keep the loading state extremely brief (single frame)
2. Use high z-index to ensure loading overlay covers everything
3. Combine with Synchronous Initial State pattern in components
4. Use opacity transitions for smooth appearance/disappearance

#### When to Use Both Patterns
- Use page-wide loading for:
  - Initial application load
  - Major route changes
  - Full page transitions

- Use Synchronous Initial State for:
  - Individual components
  - Data fetching
  - Progressive enhancement

This hybrid approach provides the best of both worlds: a polished initial load experience with the loading indicator, while maintaining instant component rendering using synchronous initial states. 