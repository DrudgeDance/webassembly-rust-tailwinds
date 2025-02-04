// Re-export layout components here
// Example: pub use container::Container;

// For now, this module is empty but structured for future layout components 


pub mod navbar;
pub mod main_content;
pub mod base;
pub mod standard;

// Re-export only what's being used
pub use base::Base;
pub use standard::Standard; 