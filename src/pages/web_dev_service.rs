use crate::components::navigation::Navigation;
use crate::components::footer::Footer;
use crate::components::section_header::SectionHeader;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

/// Web Development Service - web_development_service.rs
/// ===================
/// Web Development Services page for Stardust Software NZ
#[function_component(WebDevService)]
pub fn web_development_service() -> Html {
    // Case Study data structure
    #[derive(Clone)]
    struct CaseStudy {
        title: String,
        industry: String,
        challenge: String,
        solution: String,
        result: String,
        image_url: String,
    }

    // Define Web Development capabilities
    #[derive(Clone)]
    struct WebCapability {
        title: String,
        description: String,
        icon: String,
    }

    // Define technology stack
    #[derive(Clone)]
    struct Technology {
        name: String,
        description: String,
        image: String,
    }

    // Define case studies
    let case_studies = vec![
        CaseStudy {
            title: "Sustainable Furniture Marketplace Web Application".to_string(),
            industry: "Retail, E-commerce, Sustainability".to_string(),
            challenge: "Renewed Roots, a Wellington-based retailer specializing in recycled furniture, faced challenges with their outdated online sales platform. Issues included limited scalability, poor mobile user experience, high abandonment rates, and difficulty integrating their Shopify inventory system effectively.".to_string(),
            solution: "We developed a modern, responsive web application using Deno Fresh framework and TypeScript, seamlessly integrated with Shopify for inventory and payment processing. The solution focused on improving user experience with mobile-first design, intuitive product navigation, real-time stock synchronization, streamlined checkout, and robust filtering features tailored for sustainable furniture categories.".to_string(),
            result: "Post-launch, Renewed Roots experienced a 40% reduction in cart abandonment rates and a 32% increase in mobile conversions. Page load times improved significantly by over 50%, enhancing customer browsing experience. Within three months, overall online sales rose by 38%, accompanied by increased customer satisfaction and reduced customer support queries related to online purchasing issues.".to_string(),
            image_url: "https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fwww.architectureartdesigns.com%2Fwp-content%2Fuploads%2F2016%2F03%2F18-Remarkable-Furniture-Designs-Made-From-Recycled-Pallet-Wood-13.jpg&f=1&nofb=1&ipt=452ac8479c837abdd46a689d9a2f630bfca9caf44cfd8844be6acec775925037".to_string(),
        },
        CaseStudy {
            title: "Real-Time Bus Timetable Application".to_string(),
            industry: "Public Transport, Software Development".to_string(),
            challenge: "Developing an efficient and user-friendly web application for real-time bus timetables in Wellington was challenging due to high latency when fetching live data from the Metlink API. Users frequently experienced delays, resulting in a poor browsing experience, especially on mobile devices.".to_string(),
            solution: "We created a web application using Deno Fresh that integrates directly with the Metlink API. A dynamic loading algorithm was implemented to efficiently fetch and display bus stop data on demand, significantly reducing latency and improving real-time updates. The mobile-first design approach ensured an optimized user experience across all devices.".to_string(),
            result: "The deployment of the new bus timetable application resulted in a 60% reduction in data load latency, enhancing responsiveness and real-time accuracy. User engagement increased notably, with session durations rising by 25%. The application's intuitive interface and rapid performance received positive feedback from commuters, improving overall satisfaction with the public transport experience in Wellington.".to_string(),
            image_url: "https://images.unsplash.com/photo-1542744173-8e7e53415bb0?ixlib=rb-1.2.1&auto=format&fit=crop&w=1350&q=80".to_string(),
        },        
    ];

    // Define Web Development capabilities
    let web_capabilities = vec![
        WebCapability {
            title: "Custom Web Applications".to_string(),
            description: "Develop tailored web solutions that address your unique business requirements and provide seamless user experiences.".to_string(),
            icon: "🌐".to_string(),
        },
        WebCapability {
            title: "Responsive Design".to_string(),
            description: "Create websites that adapt perfectly to any device, ensuring optimal user experience on desktops, tablets, and mobile phones.".to_string(),
            icon: "📱".to_string(),
        },
        WebCapability {
            title: "E-commerce Development".to_string(),
            description: "Build robust online stores with secure payment processing, inventory management, and intuitive shopping experiences.".to_string(),
            icon: "🛒".to_string(),
        },
        WebCapability {
            title: "Progressive Web Apps".to_string(),
            description: "Develop applications that combine the best of web and mobile apps, with offline capabilities and native-like functionality.".to_string(),
            icon: "⚡".to_string(),
        },
        WebCapability {
            title: "CMS Implementation".to_string(),
            description: "Set up content management systems that empower your team to easily update and maintain your website without technical expertise.".to_string(),
            icon: "📄".to_string(),
        },
        WebCapability {
            title: "Web Performance Optimization".to_string(),
            description: "Enhance your website's speed and efficiency through advanced optimization techniques and best practices implementation.".to_string(),
            icon: "🚀".to_string(),
        },
    ];

    // Define technology stack
    let technologies = vec![
        Technology {
            name: "React".to_string(),
            description: "Frontend JavaScript library for building user interfaces".to_string(),
            image: "https://upload.wikimedia.org/wikipedia/commons/thumb/a/a7/React-icon.svg/1200px-React-icon.svg.png".to_string(),
        },
        Technology {
            name: "Next.js".to_string(),
            description: "React framework for production with server-side rendering".to_string(),
            image: "https://seeklogo.com/images/N/next-js-logo-8FCFF51DD2-seeklogo.com.png".to_string(),
        },
        Technology {
            name: "Rust/Yew".to_string(),
            description: "Modern, safe framework for building web applications".to_string(),
            image: "https://yew.rs/img/logo.png".to_string(),
        },
        Technology {
            name: "Node.js".to_string(),
            description: "JavaScript runtime for scalable backend services".to_string(),
            image: "https://nodejs.org/static/images/logo.svg".to_string(),
        },
        Technology {
            name: "GraphQL".to_string(),
            description: "Query language for APIs and runtime for fulfilling queries".to_string(),
            image: "https://upload.wikimedia.org/wikipedia/commons/thumb/1/17/GraphQL_Logo.svg/1200px-GraphQL_Logo.svg.png".to_string(),
        },
        Technology {
            name: "Tailwind CSS".to_string(),
            description: "Utility-first CSS framework for rapid UI development".to_string(),
            image: "https://tailwindcss.com/_next/static/media/tailwindcss-mark.d52e9897.svg".to_string(),
        },
        Technology {
            name: "Deno".to_string(),
            description: "Secure runtime for JavaScript and TypeScript".to_string(),
            image: "https://deno.com/logo.svg".to_string(),
        },
        Technology {
            name: "Fresh".to_string(),
            description: "A web framework for Deno with a focus on speed and simplicity".to_string(),
            image: "https://fresh.deno.dev/logo.svg".to_string(),
        },
        Technology {
            name: "PostgreSQL".to_string(),
            description: "Powerful, open-source object-relational database system".to_string(),
            image: "https://www.postgresql.org/media/img/about/press/elephant.png".to_string(),
        },
        Technology {
            name: "Supabase".to_string(),
            description: "Open-source Firebase alternative for building applications".to_string(),
            image: "https://supabase.com/_next/image?url=https%3A%2F%2Ffrontend-assets.supabase.com%2Fwww%2F698e585a6514%2F_next%2Fstatic%2Fmedia%2Fsupabase-logo-wordmark--dark.b36ebb5f.png&w=256&q=75&dpl=dpl_AGYA1kJymAtvMkrt8ymsAJvuQ9UR".to_string(),
        },
    ];

    html! {
        <div class="web-development-service-page">
            // Navigation
            <Navigation 
                logo_url={"https://avatars.githubusercontent.com/u/80566578?s=200&v=4"} 
                company_name={"Stardust Software NZ"} 
            />
            
            // Hero Section
            <section class="service-hero">
                <div class="service-hero-content">
                    <h1 class="service-hero-title">{"Web Development Solutions"}</h1>
                    <p class="service-hero-subtitle">{"Creating modern, responsive, and high-performance web experiences"}</p>
                </div>
                <div class="service-hero-backdrop"></div>
            </section>
            
            // Overview Section
            <section class="service-overview-section">
                <div class="container">
                    <div class="service-overview-content">
                        <div class="service-overview-text">
                            <h2>{"Elevate Your Digital Presence"}</h2>
                            <p>{"At Stardust Software, we design and develop web solutions that combine cutting-edge technology with exceptional user experiences. Our web development services help businesses establish a strong online presence, engage customers effectively, and achieve their digital transformation goals."}</p>
                            <p>{"We understand that your website or web application is often the first point of contact with your customers. Our team of skilled designers and developers creates solutions that not only look impressive but also deliver outstanding functionality, performance, and ease of use."}</p>
                            <p>{"From responsive websites to complex web applications, e-commerce platforms to content management systems, we deliver tailored solutions that align with your business objectives and exceed user expectations."}</p>
                        </div>
                        <div class="service-overview-image">
                            <img src="https://images.unsplash.com/photo-1555066931-4365d14bab8c?ixlib=rb-1.2.1&auto=format&fit=crop&w=1350&q=80" alt="Web Development" />
                        </div>
                    </div>
                </div>
            </section>
            
            // Capabilities Section
            <section class="capabilities-section">
                <div class="container">
                    <SectionHeader 
                        title="Our Web Development Capabilities" 
                        subtitle={Some("Comprehensive web solutions tailored to your business needs".to_string())}
                    />
                    
                    <div class="capabilities-grid">
                        {
                            web_capabilities.into_iter().map(|capability| {
                                html! {
                                    <div class="capability-card">
                                        <div class="capability-icon">{capability.icon}</div>
                                        <h3 class="capability-title">{capability.title}</h3>
                                        <p class="capability-description">{capability.description}</p>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </section>
            
            // Process Section
            <section class="web-process-section">
                <div class="container">
                    <SectionHeader 
                        title="Our Web Development Process" 
                        subtitle={Some("How we deliver successful web projects".to_string())}
                    />
                    
                    <div class="process-timeline">
                        <div class="process-step">
                            <div class="process-step-number">{"01"}</div>
                            <div class="process-step-content">
                                <h3>{"Discovery & Planning"}</h3>
                                <p>{"We begin by understanding your business goals, target audience, and project requirements. Our team conducts thorough research to create a strategic roadmap for your web project."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"02"}</div>
                            <div class="process-step-content">
                                <h3>{"Design & Prototyping"}</h3>
                                <p>{"Our designers create wireframes and interactive prototypes that visualize the user interface and experience. We refine these designs based on your feedback to ensure they meet your expectations."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"03"}</div>
                            <div class="process-step-content">
                                <h3>{"Development"}</h3>
                                <p>{"Our development team brings the designs to life using modern technologies and best practices. We focus on creating clean, efficient code that ensures optimal performance and maintainability."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"04"}</div>
                            <div class="process-step-content">
                                <h3>{"Testing & Quality Assurance"}</h3>
                                <p>{"We conduct comprehensive testing across different devices, browsers, and screen sizes to ensure your web solution functions flawlessly and provides a consistent experience to all users."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"05"}</div>
                            <div class="process-step-content">
                                <h3>{"Deployment & Support"}</h3>
                                <p>{"Once approved, we deploy your web solution to your hosting environment and provide ongoing maintenance and support to ensure it continues to perform optimally and evolve with your business needs."}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            
            // Case Studies Section
            <section class="case-studies-section">
                <div class="container">
                    <SectionHeader 
                        title="Web Development Success Stories" 
                        subtitle={Some("Real-world results from our web implementations".to_string())}
                    />
                    
                    <div class="case-studies-grid">
                        {
                            case_studies.into_iter().map(|case_study| {
                                html! {
                                    <div class="case-study-card">
                                        <div class="case-study-image" style={format!("background-image: url('{}')", case_study.image_url)}></div>
                                        <div class="case-study-content">
                                            <span class="case-study-industry">{case_study.industry}</span>
                                            <h3 class="case-study-title">{case_study.title}</h3>
                                            <div class="case-study-details">
                                                <div class="case-study-section">
                                                    <h4>{"Challenge"}</h4>
                                                    <p>{case_study.challenge}</p>
                                                </div>
                                                <div class="case-study-section">
                                                    <h4>{"Solution"}</h4>
                                                    <p>{case_study.solution}</p>
                                                </div>
                                                <div class="case-study-section">
                                                    <h4>{"Results"}</h4>
                                                    <p>{case_study.result}</p>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </section>
            
            // Technology Stack Section
            <section class="tech-stack-section">
                <div class="container">
                    <SectionHeader 
                        title="Our Web Development Stack" 
                        subtitle={Some("Modern technologies we leverage for web development".to_string())}
                    />
                    
                    <div class="tech-stack-grid">
                        {
                            technologies.into_iter().map(|tech| {
                                html! {
                                    <div class="tech-card">
                                        <div class="tech-image">
                                            <img src={tech.image} alt={format!("{} logo", tech.name)} />
                                        </div>
                                        <h3 class="tech-name">{tech.name}</h3>
                                        <p class="tech-description">{tech.description}</p>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </section>
            
            // FAQs Section
            <section class="faq-section">
                <div class="container">
                    <SectionHeader 
                        title="Frequently Asked Questions" 
                        subtitle={Some("Common questions about our web development services".to_string())}
                    />
                    
                    <div class="faq-grid">
                        <div class="faq-item">
                            <h3 class="faq-question">{"How long does it take to develop a website?"}</h3>
                            <p class="faq-answer">{"Project timelines vary based on complexity and scope. A simple informational website might take 4-6 weeks, while complex web applications or e-commerce platforms can take 3-6 months. During our initial consultation, we'll provide a detailed timeline based on your specific requirements."}</p>
                        </div>
                        
                        <div class="faq-item">
                            <h3 class="faq-question">{"Do you provide website maintenance after launch?"}</h3>
                            <p class="faq-answer">{"Yes, we offer comprehensive maintenance packages that include regular updates, security monitoring, performance optimization, content updates, and technical support. We recommend ongoing maintenance to ensure your website remains secure, up-to-date, and performs optimally."}</p>
                        </div>
                        
                        <div class="faq-item">
                            <h3 class="faq-question">{"How do you ensure websites are secure?"}</h3>
                            <p class="faq-answer">{"Security is integrated throughout our development process. We implement HTTPS encryption, secure coding practices, regular security updates, and protection against common vulnerabilities. For e-commerce or sites handling sensitive data, we implement additional security measures such as PCI compliance and data encryption."}</p>
                        </div>
                        
                        <div class="faq-item">
                            <h3 class="faq-question">{"Can you improve my existing website rather than building a new one?"}</h3>
                            <p class="faq-answer">{"Absolutely. We offer website audits and improvement services to enhance your existing site's design, functionality, performance, and security. After an initial assessment, we'll recommend specific improvements that align with your goals and prioritize changes that will have the most significant impact."}</p>
                        </div>
                    </div>
                </div>
            </section>
            
            // Call to Action
            <section class="cta-section">
                <div class="container">
                    <div class="cta-content">
                        <h2>{"Ready to Elevate Your Web Presence?"}</h2>
                        <p>{"Let's discuss how our web development expertise can help you create engaging digital experiences and achieve your business goals."}</p>
                        <div class="cta-buttons">
                            <Link<Route> to={Route::Contact} classes="btn btn-primary">{"Request a Free Consultation"}</Link<Route>>
                            <Link<Route> to={Route::Projects} classes="btn btn-secondary">{"View Our Web Portfolio"}</Link<Route>>
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