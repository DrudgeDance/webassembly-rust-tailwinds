#[path = "theme.light.default.rs"]
pub mod light_default;
#[path = "theme.dark.default.rs"]
pub mod dark_default;
#[path = "theme.light.spring.rs"]
pub mod light_spring;
#[path = "theme.dark.spring.rs"]
pub mod dark_spring;
#[path = "theme.light.summer.rs"]
pub mod light_summer;
#[path = "theme.dark.summer.rs"]
pub mod dark_summer;

pub use light_default::get_theme as get_light_default;
pub use dark_default::get_theme as get_dark_default;
pub use light_spring::get_theme as get_light_spring;
pub use dark_spring::get_theme as get_dark_spring;
pub use light_summer::get_theme as get_light_summer;
pub use dark_summer::get_theme as get_dark_summer; 