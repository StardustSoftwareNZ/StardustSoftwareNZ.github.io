use crate::components::navigation::Navigation;
use crate::components::hero::Hero;
use crate::components::about_section::AboutSection;
use crate::components::services_section::ServicesSection;
use crate::components::process_section::ProcessSection;
use crate::components::projects_section::ProjectsSection;
use crate::components::testimonials_section::TestimonialsSection;
use crate::components::contact_section::ContactSection;
use crate::components::footer::Footer;
use yew::prelude::*;
use web_sys::MouseEvent;

/// Home - home.rs
/// ==============
/// Fully modularized homepage component for Stardust Software NZ
#[function_component(Home)]
pub fn home() -> Html {
    let scroll_to_section = |id: &'static str| {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(element) = document.get_element_by_id(id) {
                        element.scroll_into_view_with_bool(true);
                    }
                }
            }
        })
    };

    html! {
        <div class="homepage">
            // Navigation
            <Navigation 
                logo_url={"https://avatars.githubusercontent.com/u/80566578?s=200&v=4"} 
                company_name={"Stardust Software NZ"} 
            />
            
            // Hero Section
            <Hero 
                title="Innovative Software Solutions"
                subtitle="Transforming ideas into powerful digital experiences"
                primary_button_text={Some("Our Services".to_string())}
                primary_button_onclick={Some(scroll_to_section("services"))}
                secondary_button_text={Some("Watch Demo".to_string())}
                secondary_button_onclick={None}
                scroll_down_id={Some("about".to_string())}
                scroll_down_text={Some("Discover More".to_string())}
                background_image={Some("https://images.unsplash.com/photo-1517694712202-14dd9538aa97?ixlib=rb-1.2.1&auto=format&fit=crop&w=1350&q=80".to_string())}
            />
            
            // About Section
            <AboutSection 
                id={"about"} 
                title="About Stardust Software NZ"
                paragraphs={Some(vec![
                    "At Stardust Software NZ, we blend creativity with technical expertise to deliver cutting-edge software solutions. Based in New Zealand, we work with clients globally to transform their digital presence.".to_string(),
                    "Our team of passionate developers, designers, and strategists are committed to crafting software that not only meets your business requirements but exceeds your expectations.".to_string(),
                ])}
                // Optional: Add custom statistics or an image_url
            />
            
            // Services Section
            <ServicesSection id={"services"} />
            
            // Process Section
            <ProcessSection id={"process"} />
            
            // Projects Section
            <ProjectsSection 
                id={"projects"} 
                limit={3} 
                show_cta={true} 
            />
            
            // Testimonials Section
            <TestimonialsSection 
                id={"testimonials"} 
                limit={2} 
            />
            
            // Contact Section
            <ContactSection 
                id={"contact"}
                address="123 Tech Avenue, Auckland, New Zealand"
                email="hello@stardustsoftware.nz"
                phone="+64 9 123 4567"
            />
            
            // Footer
            <Footer 
                company_name="Stardust Software NZ"
                logo_url={"https://avatars.githubusercontent.com/u/80566578?s=200&v=4"}
                tagline="Creating stellar software solutions for the modern world."
                copyright_year={2025}
            />
        </div>
    }
}