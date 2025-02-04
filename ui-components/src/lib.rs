pub mod components;
pub mod theme;
pub mod styles;
pub mod layouts;

pub use components::theme_selector02::ThemeSelector02;
pub use theme::{BaseTheme, Mode, Theme, ThemeColors};
pub use layouts::Layout;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
