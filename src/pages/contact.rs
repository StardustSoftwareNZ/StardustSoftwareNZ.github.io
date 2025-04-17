use crate::components::navigation::Navigation;
use crate::components::footer::Footer;
use crate::components::section_header::SectionHeader;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use web_sys::HtmlTextAreaElement;
use wasm_bindgen::JsCast;

/// Contact - contact.rs
/// ====================
/// Contact page for Stardust Software NZ
#[function_component(Contact)]
pub fn contact() -> Html {
    // State for form fields
    let name = use_state(|| String::new());
    let email = use_state(|| String::new());
    let phone = use_state(|| String::new());
    let subject = use_state(|| String::new());
    let message = use_state(|| String::new());
    let form_submitted = use_state(|| false);
    let form_error = use_state(|| false);

    // FAQ items
    #[derive(Clone)]
    struct FaqItem {
        question: String,
        answer: String,
    }

    // Define FAQ items
    let faq_items = vec![
        FaqItem {
            question: "How long does a typical project take?".to_string(),
            answer: "Project timelines vary depending on complexity and scope. A small website might take 4-6 weeks, while a complex enterprise application could take 4-6 months. During our initial consultation, we'll provide a detailed timeline for your specific project.".to_string(),
        },
        FaqItem {
            question: "What is your pricing structure?".to_string(),
            answer: "We offer both fixed-price project quotes and hourly rate options. For most projects, we recommend a fixed-price approach based on detailed requirements. We provide transparent quotes with no hidden costs and can tailor our pricing to fit various budget ranges.".to_string(),
        },
        FaqItem {
            question: "Do you provide ongoing support after project completion?".to_string(),
            answer: "Yes, we offer various support packages to ensure your software continues to run smoothly after launch. Our support options include regular maintenance, security updates, performance monitoring, and feature enhancements. We'll recommend the appropriate support package based on your specific needs.".to_string(),
        },
        FaqItem {
            question: "How do you handle project management and communication?".to_string(),
            answer: "We use an agile development approach with regular client check-ins. You'll have a dedicated project manager as your main point of contact, and we provide access to our project management tools where you can track progress in real-time. We typically schedule weekly updates, but can adjust based on your preferences.".to_string(),
        },
        FaqItem {
            question: "Can you work with our existing systems and technologies?".to_string(),
            answer: "Yes, we have extensive experience integrating with existing systems and working with various technology stacks. Our team is proficient in a wide range of programming languages, frameworks, and platforms. During the discovery phase, we'll assess your current infrastructure and develop an integration strategy that minimizes disruption.".to_string(),
        },
        FaqItem {
            question: "What makes Stardust Software different from other development companies?".to_string(),
            answer: "Our key differentiators include our focus on business outcomes (not just technical deliverables), our comprehensive discovery process that ensures we understand your needs deeply, our transparent communication style, and our commitment to quality through rigorous testing. We also pride ourselves on building long-term relationships with clients that extend well beyond initial project delivery.".to_string(),
        },
    ];

    // Handler for form input changes
    let handle_name_change = {
        let name = name.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let input = target.dyn_ref::<HtmlInputElement>().unwrap();
            name.set(input.value());
        })
    };

    let handle_email_change = {
        let email = email.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let input = target.dyn_ref::<HtmlInputElement>().unwrap();
            email.set(input.value());
        })
    };

    let handle_phone_change = {
        let phone = phone.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let input = target.dyn_ref::<HtmlInputElement>().unwrap();
            phone.set(input.value());
        })
    };

    let handle_subject_change = {
        let subject = subject.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let input = target.dyn_ref::<HtmlInputElement>().unwrap();
            subject.set(input.value());
        })
    };

    let handle_message_change = {
        let message = message.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let textarea = target.dyn_ref::<HtmlTextAreaElement>().unwrap();
            message.set(textarea.value());
        })
    };

    html! {
        <div class="contact-page">
            // Navigation
            <Navigation 
                logo_url={"https://avatars.githubusercontent.com/u/80566578?s=200&v=4"} 
                company_name={"Stardust Software NZ"} 
            />
            
            // Hero Section
            <section class="contact-hero">
                <div class="contact-hero-content">
                    <h1 class="contact-hero-title">{"Contact Us"}</h1>
                    <p class="contact-hero-subtitle">{"Let's start a conversation about your project"}</p>
                </div>
                <div class="contact-hero-backdrop"></div>
            </section>
            
            // Contact Information Section
            <section class="contact-info-section">
                <div class="container">
                    <div class="contact-info-grid">
                        <div class="contact-info-card">
                            <div class="contact-info-icon">
                                <svg viewBox="0 0 24 24" width="40" height="40">
                                    <path d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z" fill="#4facfe"/>
                                </svg>
                            </div>
                            <h3>{"Visit Us"}</h3>
                            <address>
                                {"Island Bay"}<br />
                                {"Wellington 6011"}<br />
                                {"New Zealand"}
                            </address>
                        </div>
                        
                        <div class="contact-info-card">
                            <div class="contact-info-icon">
                                <svg viewBox="0 0 24 24" width="40" height="40">
                                    <path d="M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z" fill="#4facfe"/>
                                </svg>
                            </div>
                            <h3>{"Email Us"}</h3>
                            <p><a href="mailto:j.r.hwood98@gmail.com">{"j.r.hwood98@gmail.com"}</a></p>
                        </div>
                        
                        <div class="contact-info-card">
                            <div class="contact-info-icon">
                                <svg viewBox="0 0 24 24" width="40" height="40">
                                    <path d="M6.62 10.79c1.44 2.83 3.76 5.14 6.59 6.59l2.2-2.2c.27-.27.67-.36 1.02-.24 1.12.37 2.33.57 3.57.57.55 0 1 .45 1 1V20c0 .55-.45 1-1 1-9.39 0-17-7.61-17-17 0-.55.45-1 1-1h3.5c.55 0 1 .45 1 1 0 1.25.2 2.45.57 3.57.11.35.03.74-.25 1.02l-2.2 2.2z" fill="#4facfe"/>
                                </svg>
                            </div>
                            <h3>{"Call Us"}</h3>
                            <p><a href="tel:+642102648190">{"+64 21026 48190"}</a></p>
                        </div>
                        
                        <div class="contact-info-card">
                            <div class="contact-info-icon">
                                <svg viewBox="0 0 24 24" width="40" height="40">
                                    <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm.31-8.86c-1.77-.45-2.34-.94-2.34-1.67 0-.84.79-1.43 2.1-1.43 1.38 0 1.9.66 1.94 1.64h1.71c-.05-1.34-.87-2.57-2.49-2.97V5H10.9v1.69c-1.51.32-2.72 1.3-2.72 2.81 0 1.79 1.49 2.69 3.66 3.21 1.95.46 2.34 1.15 2.34 1.87 0 .53-.39 1.39-2.1 1.39-1.6 0-2.23-.72-2.32-1.64H8.04c.1 1.7 1.36 2.66 2.86 2.97V19h2.34v-1.67c1.52-.29 2.72-1.16 2.73-2.77-.01-2.2-1.9-2.96-3.66-3.42z" fill="#4facfe"/>
                                </svg>
                            </div>
                            <h3>{"Business Hours"}</h3>
                            <p>{"Monday - Friday: 9AM - 6PM"}</p>
                            <p>{"Saturday - Sunday: Closed"}</p>
                        </div>
                    </div>
                </div>
            </section>
            
            // Contact Form Section
            <section class="contact-form-section">
                <div class="container">
                    <div class="contact-form-container">
                        <div class="contact-form-content">
                            <SectionHeader 
                                title="Get in Touch" 
                                subtitle={Some("We'd love to hear from you. Fill out the form below and we'll get back to you as soon as possible.".to_string())}
                            />
                            
                            <form action="https://formspree.io/f/mpwpynqy" method="POST">
                                if *form_error {
                                    <div class="form-error-message">
                                        <p>{"Please fill out all required fields."}</p>
                                    </div>
                                }
                                
                                <div class="form-row">
                                    <div class="form-group">
                                        <label for="name">{"Name"} <span class="required">{"*"}</span></label>
                                        <input 
                                            type="text" 
                                            id="name" 
                                            name="name" 
                                            value={(*name).clone()}
                                            onchange={handle_name_change}
                                            class="form-control" 
                                            required=true 
                                        />
                                    </div>
                                    <div class="form-group">
                                        <label for="email">{"Email"} <span class="required">{"*"}</span></label>
                                        <input 
                                            type="email" 
                                            id="email" 
                                            name="email" 
                                            value={(*email).clone()}
                                            onchange={handle_email_change}
                                            class="form-control" 
                                            required=true 
                                        />
                                    </div>
                                </div>
                                
                                <div class="form-row">
                                    <div class="form-group">
                                        <label for="phone">{"Phone"}</label>
                                        <input 
                                            type="tel" 
                                            id="phone" 
                                            name="phone" 
                                            value={(*phone).clone()}
                                            onchange={handle_phone_change}
                                            class="form-control" 
                                        />
                                    </div>
                                    <div class="form-group">
                                        <label for="subject">{"Subject"}</label>
                                        <input 
                                            type="text" 
                                            id="subject" 
                                            name="subject" 
                                            value={(*subject).clone()}
                                            onchange={handle_subject_change}
                                            class="form-control" 
                                        />
                                    </div>
                                </div>
                                
                                <div class="form-group">
                                    <label for="message">{"Message"} <span class="required">{"*"}</span></label>
                                    <textarea 
                                        id="message" 
                                        name="message" 
                                        value={(*message).clone()}
                                        onchange={handle_message_change}
                                        class="form-control" 
                                        rows="6" 
                                        required=true
                                    ></textarea>
                                </div>
                                
                                <div class="form-group form-checkbox">
                                    <input type="checkbox" id="privacy" name="privacy" required=true />
                                    <label for="privacy">{"I agree to the"} <a href="/privacy-policy">{"privacy policy"}</a></label>
                                </div>
                                
                                <button type="submit" class="btn btn-primary">{"Send Message"}</button>
                            </form>
                        </div>
                        <div class="contact-form-map">
                            <div class="map-container">
                                <iframe 
                                    src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d48915.11213168979!2d174.7330843158203!3d-41.2927674!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x6d38b1fc49e974cb%3A0xa00ef63a213b470!2sWellington%2C%20New%20Zealand!5e0!3m2!1sen!2sus!4v1617709430000!5m2!1sen!2sus" 
                                    width="600" 
                                    height="450" 
                                    style="border:0;" 
                                    allowfullscreen=false
                                    loading="lazy"
                                    title="Stardust Software NZ office location"
                                ></iframe>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            
            // FAQ Section
            <section class="faq-section">
                <div class="container">
                    <SectionHeader 
                        title="Frequently Asked Questions" 
                        subtitle={Some("Answers to common questions about working with us".to_string())}
                    />
                    
                    <div class="faq-grid">
                        {
                            faq_items.into_iter().map(|item| {
                                html! {
                                    <div class="faq-item">
                                        <h3 class="faq-question">{item.question}</h3>
                                        <p class="faq-answer">{item.answer}</p>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </section>
            
            // Connect Section
            <section class="connect-section">
                <div class="container">
                    <div class="connect-content">
                        <h2>{"Connect With Us"}</h2>
                        <p>{"Follow us on social media for the latest updates, insights, and tech news."}</p>
                        <div class="social-links">
                            <a href="https://x.com/jrhwood" target="_blank" rel="noopener noreferrer" class="social-link">
                                <svg viewBox="0 0 24 24" width="24" height="24">
                                    <path d="M24 4.557c-.883.392-1.832.656-2.828.775 1.017-.609 1.798-1.574 2.165-2.724-.951.564-2.005.974-3.127 1.195-.897-.957-2.178-1.555-3.594-1.555-3.179 0-5.515 2.966-4.797 6.045-4.091-.205-7.719-2.165-10.148-5.144-1.29 2.213-.669 5.108 1.523 6.574-.806-.026-1.566-.247-2.229-.616-.054 2.281 1.581 4.415 3.949 4.89-.693.188-1.452.232-2.224.084.626 1.956 2.444 3.379 4.6 3.419-2.07 1.623-4.678 2.348-7.29 2.04 2.179 1.397 4.768 2.212 7.548 2.212 9.142 0 14.307-7.721 13.995-14.646.962-.695 1.797-1.562 2.457-2.549z" fill="currentColor"/>
                                </svg>
                            </a>
                            <a href="https://www.linkedin.com/in/jrhwood/" target="_blank" rel="noopener noreferrer" class="social-link">
                                <svg viewBox="0 0 24 24" width="24" height="24">
                                    <path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z" fill="currentColor"/>
                                </svg>
                            </a>
                            <a href="https://www.facebook.com/YesHeWould" target="_blank" rel="noopener noreferrer" class="social-link">
                                <svg viewBox="0 0 24 24" width="24" height="24">
                                    <path d="M22.675 0h-21.35c-.732 0-1.325.593-1.325 1.325v21.351c0 .731.593 1.324 1.325 1.324h11.495v-9.294h-3.128v-3.622h3.128v-2.671c0-3.1 1.893-4.788 4.659-4.788 1.325 0 2.463.099 2.795.143v3.24l-1.918.001c-1.504 0-1.795.715-1.795 1.763v2.313h3.587l-.467 3.622h-3.12v9.293h6.116c.73 0 1.323-.593 1.323-1.325v-21.35c0-.732-.593-1.325-1.325-1.325z" fill="currentColor"/>
                                </svg>
                            </a>
                            <a href="https://github.com/StardustSoftwareNZ/" target="_blank" rel="noopener noreferrer" class="social-link">
                                <svg viewBox="0 0 24 24" width="24" height="24">
                                    <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" fill="currentColor"/>
                                </svg>
                            </a>
                        </div>
                    </div>
                </div>
            </section>
            
            // Footer
            <Footer 
                company_name="Stardust Software NZ"
                logo_url={"https://avatars.githubusercontent.com/u/80566578?s=200&v=4".to_string()}
                tagline="Creating stellar software solutions for the modern world."
                copyright_year={2025}
            />
        </div>
    }
}