use crate::theme::types::{Theme, ThemeColors, Season, Mode};

pub mod theme {
    use super::*;
    
    pub mod light {
        use super::*;
        
        pub mod spring {
            use super::*;
            include!("theme.light.spring.rs");
        }
        pub mod summer {
            use super::*;
            include!("theme.light.summer.rs");
        }
    }
    pub mod dark {
        use super::*;
        
        pub mod spring {
            use super::*;
            include!("theme.dark.spring.rs");
        }
        pub mod summer {
            use super::*;
            include!("theme.dark.summer.rs");
        }
    }
}

pub use theme::light::spring::get_theme as get_spring_light_theme;
pub use theme::light::summer::get_theme as get_summer_light_theme;
pub use theme::dark::spring::get_theme as get_spring_dark_theme;
pub use theme::dark::summer::get_theme as get_summer_dark_theme; 