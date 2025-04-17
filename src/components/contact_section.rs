use yew::prelude::*;
use crate::components::section_header::SectionHeader;

#[derive(Properties, PartialEq)]
pub struct ContactSectionProps {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or("Island Bay, Wellington, New Zealand".to_string())]
    pub address: String,
    #[prop_or("j.r.h.wood98@gmail.com".to_string())]
    pub email: String,
    #[prop_or("+64 21 02648190".to_string())]
    pub phone: String,
}

/// Contact section component for website
#[function_component(ContactSection)]
pub fn contact_section(props: &ContactSectionProps) -> Html {
    let id_attr = props.id.clone().unwrap_or("contact".to_string());
    
    html! {
        <>
        <section id={id_attr.clone()} class="contact-section">
            <div class="container">
                <SectionHeader 
                    title="Get in Touch"
                    subtitle={Some("Let's discuss how we can help your business grow".to_string())}
                />
                <div class="contact-grid">
                    <div class="contact-info">
                        <div class="contact-item">
                            <div class="contact-icon">
                                <svg viewBox="0 0 24 24" width="24" height="24">
                                    <path d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z" fill="#4facfe"/>
                                </svg>
                            </div>
                            <div class="contact-text">
                                <h4>{"Location"}</h4>
                                <p>{ &props.address }</p>
                            </div>
                        </div>
                        <div class="contact-item">
                            <div class="contact-icon">
                                <svg viewBox="0 0 24 24" width="24" height="24">
                                    <path d="M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z" fill="#4facfe"/>
                                </svg>
                            </div>
                            <div class="contact-text">
                                <h4>{"Email"}</h4>
                                <p>{ &props.email }</p>
                            </div>
                        </div>
                        <div class="contact-item">
                            <div class="contact-icon">
                                <svg viewBox="0 0 24 24" width="24" height="24">
                                    <path d="M6.62 10.79c1.44 2.83 3.76 5.14 6.59 6.59l2.2-2.2c.27-.27.67-.36 1.02-.24 1.12.37 2.33.57 3.57.57.55 0 1 .45 1 1V20c0 .55-.45 1-1 1-9.39 0-17-7.61-17-17 0-.55.45-1 1-1h3.5c.55 0 1 .45 1 1 0 1.25.2 2.45.57 3.57.11.35.03.74-.25 1.02l-2.2 2.2z" fill="#4facfe"/>
                                </svg>
                            </div>
                            <div class="contact-text">
                                <h4>{"Phone"}</h4>
                                <p>{ &props.phone }</p>
                            </div>
                        </div>
                        <div class="social-links">
                            // Facebook
                            <a href="https://www.facebook.com/YesHeWould" class="social-link">
                                <svg viewBox="0 0 24 24" width="24" height="24">
                                    <path d="M22.675 0h-21.35c-.732 0-1.325.593-1.325 1.325v21.351c0 .731.593 1.324 1.325 1.324h11.495v-9.294h-3.128v-3.622h3.128v-2.671c0-3.1 1.893-4.788 4.659-4.788 1.325 0 2.463.099 2.795.143v3.24l-1.918.001c-1.504 0-1.795.715-1.795 1.763v2.313h3.587l-.467 3.622h-3.12v9.293h6.116c.73 0 1.323-.593 1.323-1.325v-21.35c0-.732-.593-1.325-1.325-1.325z" fill="#4facfe"/>
                                </svg>
                            </a>
                            // LinkedIn
                            <a target="_blank" href="https://www.linkedin.com/in/jrhwood/" class="social-link">
                                <svg viewBox="0 0 24 24" width="24" height="24">
                                    <path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z" fill="#4facfe"/>
                                </svg>
                            </a>
                            // Twitter
                            <a target="_blank" href="https://x.com/jrhwood" class="social-link">
                                <svg viewBox="0 0 24 24" width="24" height="24">
                                    <path d="M24 4.557c-.883.392-1.832.656-2.828.775 1.017-.609 1.798-1.574 2.165-2.724-.951.564-2.005.974-3.127 1.195-.897-.957-2.178-1.555-3.594-1.555-3.179 0-5.515 2.966-4.797 6.045-4.091-.205-7.719-2.165-10.148-5.144-1.29 2.213-.669 5.108 1.523 6.574-.806-.026-1.566-.247-2.229-.616-.054 2.281 1.581 4.415 3.949 4.89-.693.188-1.452.232-2.224.084.626 1.956 2.444 3.379 4.6 3.419-2.07 1.623-4.678 2.348-7.29 2.04 2.179 1.397 4.768 2.212 7.548 2.212 9.142 0 14.307-7.721 13.995-14.646.962-.695 1.797-1.562 2.457-2.549z" fill="#4facfe"/>
                                </svg>
                            </a>
                        </div>
                    </div>
                    <div class="contact-form">
                        <form action="https://formspree.io/f/mpwpynqy" method="POST">
                            <div class="form-group">
                                <input type="text" id="name" name="name" placeholder="Your Name" required=true class="form-control" />
                            </div>
                            <div class="form-group">
                                <input type="email" id="email" name="email" placeholder="Your Email" required=true class="form-control" />
                            </div>
                            <div class="form-group">
                                <input type="text" id="subject" name="subject" placeholder="Subject" class="form-control" />
                            </div>
                            <div class="form-group">
                                <textarea id="message" name="message" placeholder="Your Message" required=true class="form-control"></textarea>
                            </div>
                            <button type="submit" class="btn btn-primary">{"Send Message"}</button>
                        </form>
                    </div>
                </div>
            </div>
        </section>
        </>
        }
    }