/// Components - mod.rs
/// ===================
///
/// This file is like __init__.py in Python. It is the entry point for the module.
/// It makes the each component module available to the rest of the application.
pub mod return_home;
pub use return_home::ReturnHome;

pub mod navigation;
pub mod section_header;
pub mod hero;
pub mod service_card;
pub mod project_card;
pub mod testimonial_card;
pub mod services_section;
pub mod projects_section;
pub mod testimonials_section;
pub mod contact_section;
pub mod process_section;
pub mod about_section;
pub mod footer;
pub mod scroll_to_top;