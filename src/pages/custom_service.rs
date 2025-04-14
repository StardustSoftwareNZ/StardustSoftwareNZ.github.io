use crate::components::navigation::Navigation;
use crate::components::footer::Footer;
use crate::components::section_header::SectionHeader;
use yew::prelude::*;

/// Custom Solutions Service - custom_solutions_service.rs
/// ===================
/// Custom Solutions Services page for Stardust Software NZ
#[function_component(CustomService)]
pub fn custom_solutions_service() -> Html {
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

    // Define Custom Solutions capabilities
    #[derive(Clone)]
    struct CustomCapability {
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
            title: "Scaling Palm Tree - Rust CLI for Metlink Timetable".to_string(),
            industry: "Transportation, Software Development".to_string(),
            challenge: "Commuters in Wellington needed a quick and reliable way to access accurate Metlink bus timetables directly from their terminals. Existing solutions lacked efficiency, responsiveness, and integration, often resulting in frustrating delays and outdated information.".to_string(),
            solution: "We developed Scaling Palm Tree, a custom Rust CLI application leveraging the Metlink API to provide instant access to real-time bus schedules. The application delivers a highly efficient command-line interface experience, optimized for speed and accuracy, enabling users to quickly retrieve updated timetable information directly from their desktops or servers.".to_string(),
            result: "Scaling Palm Tree significantly improved timetable accessibility, reducing data retrieval latency by over 70%. Users praised the application's simplicity and speed, resulting in enhanced commuter satisfaction and more efficient journey planning. Within weeks of deployment, it became a trusted tool for developers, commuters, and public transport enthusiasts alike.".to_string(),
            image_url: "https://images.unsplash.com/photo-1542744173-8e7e53415bb0?ixlib=rb-1.2.1&auto=format&fit=crop&w=1350&q=80".to_string(),
        },
    ];

    // Define Custom Solutions capabilities
    let custom_capabilities = vec![
        CustomCapability {
            title: "Business Process Automation".to_string(),
            description: "Streamline operations by replacing manual tasks with efficient, automated workflows tailored to your unique business processes.".to_string(),
            icon: "⚙️".to_string(),
        },
        CustomCapability {
            title: "Legacy System Modernization".to_string(),
            description: "Transform outdated systems into modern, scalable solutions while preserving critical business logic and data.".to_string(),
            icon: "🔄".to_string(),
        },
        CustomCapability {
            title: "Integration Solutions".to_string(),
            description: "Connect disparate systems and data sources to create a unified ecosystem that enhances information flow and collaboration.".to_string(),
            icon: "🔌".to_string(),
        },
        CustomCapability {
            title: "Data Management Solutions".to_string(),
            description: "Develop custom databases and analytics platforms that transform raw data into actionable business insights.".to_string(),
            icon: "📊".to_string(),
        },
        CustomCapability {
            title: "Industry-Specific Applications".to_string(),
            description: "Create specialized software solutions designed to address the unique challenges and requirements of your industry.".to_string(),
            icon: "🏭".to_string(),
        },
        CustomCapability {
            title: "IoT & Embedded Systems".to_string(),
            description: "Build connected solutions that integrate hardware devices with software applications for real-time monitoring and control.".to_string(),
            icon: "📱".to_string(),
        },
    ];

    // Define technology stack
    let technologies = vec![
        Technology {
            name: "Python".to_string(),
            description: "Versatile language for backend, data processing".to_string(),
            image: "https://upload.wikimedia.org/wikipedia/commons/thumb/c/c3/Python-logo-notext.svg/1200px-Python-logo-notext.svg.png".to_string(),
        },
        Technology {
            name: "Java".to_string(),
            description: "Enterprise-grade application development".to_string(),
            image: "https://upload.wikimedia.org/wikipedia/en/thumb/3/30/Java_programming_language_logo.svg/1200px-Java_programming_language_logo.svg.png".to_string(),
        },
        Technology {
            name: "JavaScript".to_string(),
            description: "Dynamic web applications and server-side development".to_string(),
            image: "https://upload.wikimedia.org/wikipedia/commons/thumb/9/99/Unofficial_JavaScript_logo_2.svg/1200px-Unofficial_JavaScript_logo_2.svg.png".to_string(),
        },
        Technology {
            name: "PostgreSQL".to_string(),
            description: "Advanced open source database system".to_string(),
            image: "https://upload.wikimedia.org/wikipedia/commons/thumb/2/29/Postgresql_elephant.svg/1200px-Postgresql_elephant.svg.png".to_string(),
        },
        Technology {
            name: "Rust".to_string(),
            description: "High-performance, memory-safe applications".to_string(),
            image: "https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1200px-Rust_programming_language_black_logo.svg.png".to_string(),
        },
        Technology {
            name: "Haskell".to_string(),
            description: "Functional programming for complex algorithms".to_string(),
            image: "https://www.haskell.org/img/haskell-logo.svg".to_string(),
        }
    ];

    html! {
        <div class="custom-solutions-page">
            // Navigation
            <Navigation 
                logo_url={"https://avatars.githubusercontent.com/u/80566578?s=200&v=4"} 
                company_name={"Stardust Software NZ"} 
            />
            
            // Hero Section
            <section class="service-hero">
                <div class="service-hero-content">
                    <h1 class="service-hero-title">{"Custom Software Solutions"}</h1>
                    <p class="service-hero-subtitle">{"Tailored applications designed to solve your unique business challenges"}</p>
                </div>
                <div class="service-hero-backdrop"></div>
            </section>
            
            // Overview Section
            <section class="service-overview-section">
                <div class="container">
                    <div class="service-overview-content">
                        <div class="service-overview-text">
                            <h2>{"Software Built for Your Specific Needs"}</h2>
                            <p>{"At Stardust Software, we specialize in developing custom solutions that address the unique challenges and requirements of your business. Unlike off-the-shelf software that forces you to adapt your processes to its limitations, our custom applications are designed specifically for your organization's workflow, data needs, and growth objectives."}</p>
                            <p>{"Our team of experienced analysts, architects, and developers work closely with you to understand your business processes, identify pain points, and design solutions that streamline operations, improve efficiency, and provide competitive advantages. We focus on creating intuitive, scalable applications that solve today's problems while adapting to tomorrow's needs."}</p>
                            <p>{"From enterprise workflow systems and specialized industry applications to data management platforms and legacy system modernization, we deliver tailored solutions that transform how your business operates and create lasting value."}</p>
                        </div>
                        <div class="service-overview-image">
                            <img src="https://images.unsplash.com/photo-1581291518857-4e27b48ff24e?ixlib=rb-1.2.1&auto=format&fit=crop&w=1350&q=80" alt="Custom Software Development" />
                        </div>
                    </div>
                </div>
            </section>
            
            // Capabilities Section
            <section class="capabilities-section">
                <div class="container">
                    <SectionHeader 
                        title="Our Custom Development Capabilities" 
                        subtitle={Some("Comprehensive solutions tailored to your specific requirements".to_string())}
                    />
                    
                    <div class="capabilities-grid">
                        {
                            custom_capabilities.into_iter().map(|capability| {
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
            <section class="custom-process-section">
                <div class="container">
                    <SectionHeader 
                        title="Our Custom Solution Development Process" 
                        subtitle={Some("How we deliver successful custom software projects".to_string())}
                    />
                    
                    <div class="process-timeline">
                        <div class="process-step">
                            <div class="process-step-number">{"01"}</div>
                            <div class="process-step-content">
                                <h3>{"Discovery & Analysis"}</h3>
                                <p>{"We begin by thoroughly understanding your business processes, challenges, and objectives. Our analysts work with your team to document requirements, identify pain points, and establish clear success criteria for the project."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"02"}</div>
                            <div class="process-step-content">
                                <h3>{"Solution Design"}</h3>
                                <p>{"Our architects design a comprehensive solution that addresses your needs while ensuring scalability, security, and usability. We create detailed specifications and prototypes to validate the approach before development begins."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"03"}</div>
                            <div class="process-step-content">
                                <h3>{"Iterative Development"}</h3>
                                <p>{"We follow an agile development approach, building your solution in incremental sprints with regular demonstrations and feedback cycles. This ensures the final product aligns perfectly with your expectations and business needs."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"04"}</div>
                            <div class="process-step-content">
                                <h3>{"Testing & Quality Assurance"}</h3>
                                <p>{"Our QA team rigorously tests the solution to ensure it meets all functional requirements, performs optimally under various conditions, and maintains data integrity and security standards."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"05"}</div>
                            <div class="process-step-content">
                                <h3>{"Deployment & Ongoing Support"}</h3>
                                <p>{"We carefully manage the deployment process, including data migration, user training, and system integration. After launch, we provide ongoing maintenance, support, and enhancements to ensure long-term success."}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            
            // Case Studies Section
            <section class="case-studies-section">
                <div class="container">
                    <SectionHeader 
                        title="Custom Solution Success Stories" 
                        subtitle={Some("Real-world results from our custom implementations".to_string())}
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
                        title="Our Custom Solution Technology Stack" 
                        subtitle={Some("Powerful technologies we leverage for custom development".to_string())}
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
                        subtitle={Some("Common questions about our custom development services".to_string())}
                    />
                    
                    <div class="faq-grid">
                        <div class="faq-item">
                            <h3 class="faq-question">{"Why should we invest in custom software instead of using off-the-shelf solutions?"}</h3>
                            <p class="faq-answer">{"While off-the-shelf software can work well for standard processes, custom solutions offer several advantages: they're built specifically for your unique workflows, eliminate the compromises required with generic software, provide competitive advantages through differentiation, integrate seamlessly with your existing systems, scale according to your specific needs, and give you full control over features and enhancements. The initial investment is often offset by increased efficiency, reduced workarounds, and perfect alignment with business processes."}</p>
                        </div>
                        
                        <div class="faq-item">
                            <h3 class="faq-question">{"How long does custom software development take?"}</h3>
                            <p class="faq-answer">{"Development timelines vary significantly based on complexity and scope. Simple applications might be completed in 3-4 months, while complex enterprise systems can take 8-12 months or more. We use an agile approach to deliver functionality incrementally, allowing you to start utilizing key features earlier in the development process. During our discovery phase, we'll provide a detailed timeline based on your specific requirements."}</p>
                        </div>
                        
                        <div class="faq-item">
                            <h3 class="faq-question">{"What happens if we need changes after the software is developed?"}</h3>
                            <p class="faq-answer">{"Change is inevitable in business, and your software should evolve accordingly. We offer ongoing maintenance and enhancement services to ensure your custom solution continues to meet your needs as your business grows and changes. Since you own the software, you have complete control over its evolution. We can implement new features, integrate with additional systems, or modify existing functionality as your requirements change."}</p>
                        </div>
                        
                        <div class="faq-item">
                            <h3 class="faq-question">{"How do you ensure the quality and security of custom software?"}</h3>
                            <p class="faq-answer">{"Quality and security are integrated throughout our development process. We follow industry best practices for secure coding, conduct thorough code reviews, and implement automated testing at multiple levels (unit, integration, system). Our QA team performs comprehensive manual testing to ensure functionality and usability. For security, we implement appropriate authentication and authorization controls, data encryption, input validation, and protection against common vulnerabilities. For applications handling sensitive data, we can also conduct third-party security audits."}</p>
                        </div>
                    </div>
                </div>
            </section>
            
            // Call to Action
            <section class="cta-section">
                <div class="container">
                    <div class="cta-content">
                        <h2>{"Ready to Transform Your Business with Custom Software?"}</h2>
                        <p>{"Let's discuss how a tailored solution can address your unique challenges and create lasting value for your organization."}</p>
                        <div class="cta-buttons">
                            <a href="/contact" class="btn btn-primary">{"Schedule a Consultation"}</a>
                            <a href="/custom-solutions-assessment" class="btn btn-secondary">{"Request a Free Needs Assessment"}</a>
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