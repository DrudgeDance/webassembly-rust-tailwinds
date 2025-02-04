pub mod components;
pub mod layouts;
pub mod styles;
pub mod theme;
pub mod defaults;

// Re-export commonly used components
pub use components::Hello;
pub use components::ThemeSelector;
pub use theme::*;

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
