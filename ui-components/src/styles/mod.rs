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