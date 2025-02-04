// Common styles and theme configurations

#[allow(dead_code)]
pub mod theme {
    #[allow(dead_code)]
    pub struct Colors {
        pub primary: &'static str,
        pub background: &'static str,
        pub text: &'static str,
    }

    #[allow(dead_code)]
    pub const DEFAULT_THEME: Colors = Colors {
        primary: "#333",
        background: "#f0f0f0",
        text: "#333",
    };
}

#[allow(dead_code)]
pub mod constants {
    pub const BORDER_RADIUS: &str = "8px";
    pub const BOX_SHADOW: &str = "0 2px 4px rgba(0,0,0,0.1)";
    pub const FONT_FAMILY: &str = "Arial, sans-serif";
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn theme_css() {
    // Load theme CSS
    use std::include_str;
    let style = include_str!("theme.css");
    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap();
    let head = document.head().unwrap();
    let style_element = document
        .create_element("style")
        .unwrap();
    style_element.set_text_content(Some(style));
    head.append_child(&style_element).unwrap();
} 