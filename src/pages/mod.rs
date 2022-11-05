/// Pages - mod.rs
/// ==============
/// This file is like __init__.py in Python. It is the entry point for the module.
/// It makes the each page module available to the rest of the application.
pub mod home;
pub use home::Home;

pub mod about;
pub use about::About;

pub mod error;
pub use error::Error;

pub mod secure;
pub use secure::Secure;
