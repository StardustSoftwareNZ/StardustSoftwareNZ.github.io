use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    #[prop_or("Stardust Software NZ".to_string())]
    pub company_name: String,
    #[prop_or_default]
    pub logo_url: Option<String>,
    #[prop_or("Creating stellar software solutions for the modern world.".to_string())]
    pub tagline: String,
    #[prop_or(2025)]
    pub copyright_year: u32,
}

/// Footer component for website
#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let logo_url = props.logo_url.clone().unwrap_or("https://avatars.githubusercontent.com/u/80566578?s=200&v=4".to_string());
    
    html! {
        <footer class="footer">
            <div class="container">
                <div class="footer-grid">
                    <div class="footer-info">
                        <div class="footer-logo">
                            <img src={logo_url} alt={format!("{} Logo", props.company_name)} />
                            <h3>{ &props.company_name }</h3>
                        </div>
                        <p class="footer-tagline">{ &props.tagline }</p>
                    </div>
                    <div class="footer-links">
                        <h4>{"Quick Links"}</h4>
                        <ul>
                            <li><a href="#">{"Home"}</a></li>
                            <li><a href="#about">{"About"}</a></li>
                            <li><a href="#services">{"Services"}</a></li>
                            <li><a href="#projects">{"Projects"}</a></li>
                            <li><a href="#contact">{"Contact"}</a></li>
                        </ul>
                    </div>
                    <div class="footer-links">
                        <h4>{"Services"}</h4>
                        <ul>
                            <li><a href="#">{"Web Development"}</a></li>
                            <li><a href="#">{"Mobile Applications"}</a></li>
                            <li><a href="#">{"Cloud Solutions"}</a></li>
                            <li><a href="#">{"Custom Software"}</a></li>
                            <li><a href="#">{"UI/UX Design"}</a></li>
                        </ul>
                    </div>
                    <div class="footer-newsletter">
                        <h4>{"Stay Updated"}</h4>
                        <p>{"Subscribe to our newsletter for the latest news and insights."}</p>
                        <form class="newsletter-form">
                            <input type="email" placeholder="Your Email" required=true />
                            <button type="submit">{"Subscribe"}</button>
                        </form>
                    </div>
                </div>
                <div class="footer-bottom">
                    <p class="copyright">{format!("© {} {} All Rights Reserved.", props.copyright_year, props.company_name)}</p>
                    <div class="footer-legal">
                        <a href="#">{"Privacy Policy"}</a>
                        <a href="#">{"Terms of Service"}</a>
                    </div>
                </div>
            </div>
        </footer>
    }
}