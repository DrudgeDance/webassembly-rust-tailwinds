mod types;
pub mod themes;
mod router;
mod provider;
mod utils;

pub use types::{Theme, ThemeColors, Season, Mode};
pub use router::ThemeRouter;
pub use provider::ThemeProvider;
pub use utils::{get_system_mode, get_theme_variant};

lazy_static::lazy_static! {
    pub static ref SPRING_LIGHT_THEME: Theme = themes::get_spring_light_theme();
} 