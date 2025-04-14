use yew::prelude::*;
use crate::api::testimonials::Testimonial;

#[derive(Properties, PartialEq)]
pub struct TestimonialCardProps {
    pub testimonial: Testimonial,
}

/// Testimonial card component to display client feedback
#[function_component(TestimonialCard)]
pub fn testimonial_card(props: &TestimonialCardProps) -> Html {
    let testimonial = &props.testimonial;
    let author_image_style = format!("background-image: url('{}')", testimonial.author_image);
    
    html! {
        <div class="testimonial-card">
            <div class="testimonial-quote">{""}</div>
            <p class="testimonial-text">{ &testimonial.text }</p>
            <div class="testimonial-author">
                <div class="author-image" style={author_image_style}></div>
                <div class="author-info">
                    <h4 class="author-name">{ &testimonial.author_name }</h4>
                    <p class="author-company">
                        { format!("{}, {}", testimonial.author_title, testimonial.author_company) }
                    </p>
                </div>
            </div>
        </div>
    }
}