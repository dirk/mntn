pub mod common;

#[cfg(target_os = "macos")]
pub use platform::macos::{
    Application,
    Menu,
    MenuItem,
    Window
};

#[cfg(target_os = "macos")]
mod macos;