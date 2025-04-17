use yew::prelude::*;
use crate::api::testimonials::{fetch_testimonials, get_hardcoded_testimonials};
use crate::components::section_header::SectionHeader;
use crate::components::testimonial_card::TestimonialCard;
use wasm_bindgen_futures::spawn_local;

#[derive(Properties, PartialEq)]
pub struct TestimonialsSectionProps {
    #[prop_or(3)]
    pub limit: usize,
    #[prop_or_default]
    pub id: Option<String>,
}

/// Testimonials section component to display client feedback
#[function_component(TestimonialsSection)]
pub fn testimonials_section(props: &TestimonialsSectionProps) -> Html {
    // State for storing testimonials
    let testimonials = use_state(|| Vec::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    
    // Fetch testimonials on component mount
    {
        let testimonials = testimonials.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    match fetch_testimonials().await {
                        Ok(fetched_testimonials) => {
                            testimonials.set(fetched_testimonials);
                            loading.set(false);
                        },
                        Err(err) => {
                            // Log the error
                            web_sys::console::error_1(&err);
                            // Fallback to hardcoded testimonials
                            testimonials.set(get_hardcoded_testimonials());
                            error.set(Some(format!("Error: {:?}", err)));
                            loading.set(false);
                        }
                    }
                });
                
                || ()
            },
            (), // Empty dependencies - only run once on mount
        );
    }
    
    // Limit testimonials based on the prop
    let display_testimonials = testimonials
        .iter()
        .take(props.limit)
        .cloned()
        .collect::<Vec<_>>();
    
    let id_attr = props.id.clone().unwrap_or("testimonials".to_string());
    
    html! {
        <section id={id_attr} class="testimonials-section">
            <div class="container">
                <SectionHeader
                    title="Client Testimonials"
                    subtitle={Some("What our clients say about us".to_string())}
                />
                
                {
                    if *loading {
                        html! {
                            <div class="loading-indicator">
                                <p>{"Loading testimonials..."}</p>
                            </div>
                        }
                    } else {
                        html! {
                            <>
                                {
                                    if let Some(err_msg) = &*error {
                                        html! {
                                            <div class="error-message" style="margin-bottom: 1rem; font-size: 0.9rem; color: #666;">
                                                <p>{"Using fallback testimonial data"}</p>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                
                                <div class="testimonials-slider">
                                    {
                                        display_testimonials.into_iter().map(|testimonial| {
                                            html! {
                                                <TestimonialCard key={testimonial.id.clone()} testimonial={testimonial.clone()} />
                                            }
                                        }).collect::<Html>()
                                    }
                                </div>
                            </>
                        }
                    }
                }
            </div>
        </section>
    }
}