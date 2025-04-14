use yew::prelude::*;
use crate::api::testimonials::get_testimonials;
use crate::components::section_header::SectionHeader;
use crate::components::testimonial_card::TestimonialCard;

#[derive(Properties, PartialEq)]
pub struct TestimonialsSectionProps {
    #[prop_or(2)]
    pub limit: usize,
    #[prop_or_default]
    pub id: Option<String>,
}

/// Testimonials section component to display client feedback
#[function_component(TestimonialsSection)]
pub fn testimonials_section(props: &TestimonialsSectionProps) -> Html {
    // Get all testimonials and limit them based on the prop
    let all_testimonials = get_testimonials();
    let testimonials = all_testimonials.into_iter().take(props.limit).collect::<Vec<_>>();
    
    let id_attr = props.id.clone().unwrap_or("testimonials".to_string());
    
    html! {
        <section id={id_attr} class="testimonials-section">
            <div class="container">
                <SectionHeader 
                    title="Client Testimonials" 
                    subtitle={Some("What our clients say about us".to_string())}
                />
                
                <div class="testimonials-slider">
                    {
                        testimonials.into_iter().map(|testimonial| {
                            html! {
                                <TestimonialCard key={testimonial.id.clone()} testimonial={testimonial.clone()} />
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </section>
    }
}