use frontend::App;

fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    
    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::default());
    
    yew::Renderer::<App>::new().render();
} 