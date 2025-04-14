use crate::components::navigation::Navigation;
use crate::components::footer::Footer;
use crate::components::section_header::SectionHeader;
use yew::prelude::*;

/// About Us - about.rs
/// ===================
/// About page for Stardust Software NZ
#[function_component(About)]
pub fn about() -> Html {
    // Team member data structure
    #[derive(Clone)]
    struct TeamMember {
        name: String,
        role: String,
        bio: String,
        image_url: String,
    }

    // Company values data structure
    #[derive(Clone)]
    struct CompanyValue {
        title: String,
        description: String,
        icon: String,
    }

    // Define team members
    let team_members = vec![
        TeamMember {
            name: "Jesse Wood".to_string(),
            role: "CEO & Founder".to_string(),
            bio: "BE(Hons) in Software Engineering and PhD Candidate, Jesse founded Stardust Software leave the world a better place than he found it.".to_string(),
            image_url: "https://pbs.twimg.com/profile_images/1904799151331958786/KxV1kqJ7_400x400.jpg".to_string(),
        }
    ];

    // Define company values
    let company_values = vec![
        CompanyValue {
            title: "Innovation".to_string(),
            description: "We push boundaries and challenge conventions to create software solutions that lead the industry.".to_string(),
            icon: "💡".to_string(),
        },
        CompanyValue {
            title: "Quality".to_string(),
            description: "We maintain the highest standards in our code, design, and overall project delivery.".to_string(),
            icon: "✨".to_string(),
        },
        CompanyValue {
            title: "Collaboration".to_string(),
            description: "We believe great work happens through partnership with our clients and within our team.".to_string(),
            icon: "🤝".to_string(),
        },
        CompanyValue {
            title: "Integrity".to_string(),
            description: "We operate with transparency and honesty in all our business relationships.".to_string(),
            icon: "🛡️".to_string(),
        },
        CompanyValue {
            title: "User-Centric".to_string(),
            description: "We design and develop with the end user in mind, creating intuitive and accessible experiences.".to_string(),
            icon: "👥".to_string(),
        },
        CompanyValue {
            title: "Continuous Improvement".to_string(),
            description: "We're committed to learning, growth, and becoming better at what we do every day.".to_string(),
            icon: "📈".to_string(),
        },
    ];

    html! {
        <div class="about-page">
            // Navigation
            <Navigation 
                logo_url={"https://avatars.githubusercontent.com/u/80566578?s=200&v=4"} 
                company_name={"Stardust Software NZ"} 
            />
            
            // Hero Section
            <section class="about-hero">
                <div class="about-hero-content">
                    <h1 class="about-hero-title">{"About Stardust Software"}</h1>
                    <p class="about-hero-subtitle">{"Creating stellar software experiences since 2015"}</p>
                </div>
                <div class="about-hero-backdrop"></div>
            </section>
            
            // Our Story Section
            <section class="our-story-section">
                <div class="container">
                    <SectionHeader 
                        title="Our Story" 
                        subtitle={Some("The journey behind Stardust Software".to_string())}
                    />
                    
                    <div class="story-content">
                        <div class="story-image">
                            <img src="https://images.unsplash.com/photo-1522071820081-009f0129c71c?ixlib=rb-1.2.1&auto=format&fit=crop&w=1350&q=80" alt="Stardust Software team" />
                        </div>
                        <div class="story-text">
                            <p>{"Stardust Software is a boutique software contracting company based in Wellington, New Zealand, specializing in cutting-edge AI solutions and research-driven development. Founded and operated by Jesse Wood, the company delivers bespoke machine learning systems with a focus on clarity, performance, and innovation."}</p>
                            <p>{"Jesse, the sole engineer behind Stardust Software, holds a degree in Software Engineering from Victoria University of Wellington and is currently pursuing a PhD in Artificial Intelligence. His work bridges the gap between academic research and real-world application, with a passion for making intelligent systems interpretable and impactful."}</p>
                            <p>{"At Stardust Software, every project is treated as an opportunity to explore new frontiers in software and AI. From clean, maintainable codebases to state-of-the-art models, the company is committed to building tools that are not only functional, but futuristic"}</p>
                        </div>
                    </div>
                </div>
            </section>
            
            // Our Values Section
            <section class="values-section">
                <div class="container">
                    <SectionHeader 
                        title="Our Values" 
                        subtitle={Some("The principles that guide our work".to_string())}
                    />
                    
                    <div class="values-grid">
                        {
                            company_values.into_iter().map(|value| {
                                html! {
                                    <div class="value-card">
                                        <div class="value-icon">{value.icon}</div>
                                        <h3 class="value-title">{value.title}</h3>
                                        <p class="value-description">{value.description}</p>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </section>
            
            // Team Section
            <section class="team-section">
                <div class="container">
                    <SectionHeader 
                        title="Our Team" 
                        subtitle={Some("Meet the people behind Stardust Software".to_string())}
                    />
                    
                    <div class="team-grid">
                        {
                            team_members.into_iter().map(|member| {
                                html! {
                                    <div class="team-member">
                                        <div class="team-member-image" style={format!("background-image: url('{}')", member.image_url)}></div>
                                        <div class="team-member-info">
                                            <h3 class="team-member-name">{member.name}</h3>
                                            <p class="team-member-role">{member.role}</p>
                                            <p class="team-member-bio">{member.bio}</p>
                                            <div class="team-member-social">
                                                <a href="#" class="social-link"><i class="fab fa-linkedin"></i></a>
                                                <a href="#" class="social-link"><i class="fab fa-twitter"></i></a>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </section>
            
            // Stats Section
            <section class="stats-section">
                <div class="container">
                    <div class="stats-grid">
                        <div class="stat-item">
                            <div class="stat-number">{"1+"}</div>
                            <div class="stat-label">{"Projects Completed"}</div>
                        </div>
                        <div class="stat-item">
                            <div class="stat-number">{"1+"}</div>
                            <div class="stat-label">{"Team Members"}</div>
                        </div>
                        <div class="stat-item">
                            <div class="stat-number">{"4+"}</div>
                            <div class="stat-label">{"Years of Excellence"}</div>
                        </div>
                        <div class="stat-item">
                            <div class="stat-number">{"3+"}</div>
                            <div class="stat-label">{"Happy Clients"}</div>
                        </div>
                    </div>
                </div>
            </section>
            
            // Office Section
            <section class="office-section">
                <div class="container">
                    <SectionHeader 
                        title="Our Office" 
                        subtitle={Some("Where the magic happens".to_string())}
                    />
                    
                    <div class="office-content">
                        <div class="office-gallery">
                            <div class="office-image large">
                                <img src="https://images.unsplash.com/photo-1497366811353-6870744d04b2?ixlib=rb-1.2.1&auto=format&fit=crop&w=1350&q=80" alt="Stardust Software office" />
                            </div>
                            <div class="office-image">
                                <img src="https://images.unsplash.com/photo-1497215842964-222b430dc094?ixlib=rb-1.2.1&auto=format&fit=crop&w=600&q=80" alt="Stardust Software office" />
                            </div>
                            <div class="office-image">
                                <img src="https://images.unsplash.com/photo-1527192491265-7e15c55b1ed2?ixlib=rb-1.2.1&auto=format&fit=crop&w=600&q=80" alt="Stardust Software office" />
                            </div>
                        </div>
                        <div class="office-info">
                            <h3>{"Located in the Heart of Wellington"}</h3>
                            <p>{"Our modern office space is designed to foster creativity, collaboration, and productivity. We believe that a positive work environment leads to exceptional results."}</p>
                            <p>{"Visit us at:"}</p>
                            <address>
                                {"Island Bay"}<br />
                                {"Wellington"}<br />
                                {"New Zealand"}
                            </address>
                        </div>
                    </div>
                </div>
            </section>
            
            // Call to Action
            <section class="about-cta-section">
                <div class="container">
                    <div class="about-cta-content">
                        <h2>{"Ready to Work With Us?"}</h2>
                        <p>{"Let's discuss how we can help bring your software vision to life."}</p>
                        <div class="about-cta-buttons">
                            <a href="/contact" class="btn btn-primary">{"Get in Touch"}</a>
                            <a href="/services" class="btn btn-secondary">{"Explore Our Services"}</a>
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