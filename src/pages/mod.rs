/// Pages - mod.rs
/// ==============
/// These pages correspond to the routes defined in the Route module.
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

pub mod projects;
pub use projects::Projects;

pub mod contact;
pub use contact::Contact;

pub mod ai_service;
pub use ai_service::AiService;

pub mod web_dev_service;
pub use web_dev_service::WebDevService;

pub mod cloud_service;
pub use cloud_service::CloudService;

pub mod custom_service;
pub use custom_service::CustomService;

pub mod mobile_service;
pub use mobile_service::MobileService;
