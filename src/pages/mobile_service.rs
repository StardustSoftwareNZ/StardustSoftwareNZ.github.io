use crate::components::navigation::Navigation;
use crate::components::footer::Footer;
use crate::components::section_header::SectionHeader;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

/// Mobile Applications Service - mobile_applications_service.rs
/// ===================
/// Mobile Applications Service page for Stardust Software NZ
#[function_component(MobileService)]
pub fn mobile_applications_service() -> Html {
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

    // Define Mobile capabilities
    #[derive(Clone)]
    struct MobileCapability {
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
            title: "Ionic Scholar - Mobile Research Companion".to_string(),
            industry: "Education Technology, Mobile Application Development".to_string(),
            challenge: "Academic researchers and students needed a seamless solution for managing their reading progress, organizing academic citations, and taking structured notes on scholarly papers. Existing tools lacked integration, accessibility, and ease-of-use, leading to fragmented workflows and inefficient research processes.".to_string(),
            solution: "We developed Ionic Scholar, a mobile application using the Ionic React framework, designed specifically to streamline research management. The app enables users to track reading progress through detailed citation management, store and categorize references efficiently, and take structured notes directly linked to academic papers, all accessible from a unified, intuitive mobile interface.".to_string(),
            result: "Ionic Scholar successfully simplified academic workflows, leading to improved research productivity and organization among users. Post-launch feedback indicated a 45% increase in research efficiency, substantial improvements in citation organization, and greater satisfaction with mobile-based academic workflows. The app quickly became a valuable tool in academic communities for structured research management.".to_string(),
            image_url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwww.higheredjobs.com%2Fimages%2Farticles%2Farticle_3575_1.jpg&f=1&nofb=1&ipt=7fce5546a09c0bd9e59272394de4254a1965bcf35be8051763a289fea8a0006d".to_string(),
        },
    ];

    // Define Mobile capabilities
    let mobile_capabilities = vec![
        MobileCapability {
            title: "Native App Development".to_string(),
            description: "Build high-performance, feature-rich applications optimized specifically for iOS and Android platforms.".to_string(),
            icon: "📱".to_string(),
        },
        MobileCapability {
            title: "Cross-Platform Solutions".to_string(),
            description: "Develop apps that work seamlessly across multiple platforms using frameworks like Flutter and React Native.".to_string(),
            icon: "🔄".to_string(),
        },
        MobileCapability {
            title: "Mobile UX/UI Design".to_string(),
            description: "Create intuitive, engaging, and accessible mobile interfaces that delight users and drive engagement.".to_string(),
            icon: "🎨".to_string(),
        },
        MobileCapability {
            title: "Offline Functionality".to_string(),
            description: "Implement robust offline capabilities that allow apps to function seamlessly in areas with limited connectivity.".to_string(),
            icon: "⚡".to_string(),
        },
        MobileCapability {
            title: "App Modernization".to_string(),
            description: "Transform legacy mobile applications into modern, feature-rich solutions using the latest technologies.".to_string(),
            icon: "🔧".to_string(),
        },
        MobileCapability {
            title: "Integration Services".to_string(),
            description: "Connect your mobile apps with existing enterprise systems, third-party services, and APIs.".to_string(),
            icon: "🔌".to_string(),
        },
    ];

    // Define technology stack
    let technologies = vec![
        Technology {
            name: "Ionic".to_string(),
            description: "Build Native and Progressive Web Apps from a single code base with Ionic React.".to_string(),
            image: "https://ionicframework.com/img/react/logo.png".to_string(),
        },
        Technology {
            name: "Flutter".to_string(),
            description: "Cross-platform UI toolkit by Google".to_string(),
            image: "https://storage.googleapis.com/cms-storage-bucket/6e19fee6b47b36ca613f.png".to_string(),
        },
        Technology {
            name: "React Native".to_string(),
            description: "Framework for building native apps using React".to_string(),
            image: "https://upload.wikimedia.org/wikipedia/commons/thumb/a/a7/React-icon.svg/1200px-React-icon.svg.png".to_string(),
        },
        Technology {
            name: "Firebase".to_string(),
            description: "Google's mobile app development platform".to_string(),
            image: "https://firebase.google.com/downloads/brand-guidelines/PNG/logo-logomark.png".to_string(),
        },
        Technology {
            name: "Supabase".to_string(),
            description: "Open-source Firebase alternative for building applications".to_string(),
            image: "https://logowik.com/content/uploads/images/supabase-icon9119.logowik.com.webp".to_string(),
        },
        Technology {
            name: "PostgreSQL".to_string(),
            description: "Open-source relational database management system".to_string(),
            image: "https://www.postgresql.org/media/img/about/press/elephant.png".to_string(),
        },
    ];

    html! {
        <div class="mobile-applications-page">
            // Navigation
            <Navigation 
                logo_url={"https://avatars.githubusercontent.com/u/80566578?s=200&v=4"} 
                company_name={"Stardust Software NZ"} 
            />
            
            // Hero Section
            <section class="service-hero">
                <div class="service-hero-content">
                    <h1 class="service-hero-title">{"Mobile Application Development"}</h1>
                    <p class="service-hero-subtitle">{"Creating powerful, intuitive mobile experiences for iOS and Android"}</p>
                </div>
                <div class="service-hero-backdrop"></div>
            </section>
            
            // Overview Section
            <section class="service-overview-section">
                <div class="container">
                    <div class="service-overview-content">
                        <div class="service-overview-text">
                            <h2>{"Engage Users on the Devices They Use Most"}</h2>
                            <p>{"At Stardust Software, we build mobile applications that connect businesses with their customers and empower workforces with tools that drive productivity. Our mobile solutions combine intuitive design with powerful functionality to deliver exceptional experiences on smartphones and tablets."}</p>
                            <p>{"We understand that successful mobile applications require more than just coding expertise. Our team of designers, developers, and UX specialists work together to create mobile experiences that are not only visually appealing but also highly functional, secure, and optimized for performance."}</p>
                            <p>{"Whether you need a native application for iOS or Android, a cross-platform solution that works across multiple devices, or a mobile companion to your existing web platform, we deliver mobile solutions that help you achieve your business objectives."}</p>
                        </div>
                        <div class="service-overview-image">
                            <img src="https://images.unsplash.com/photo-1512941937669-90a1b58e7e9c?ixlib=rb-1.2.1&auto=format&fit=crop&w=1350&q=80" alt="Mobile App Development" />
                        </div>
                    </div>
                </div>
            </section>
            
            // Capabilities Section
            <section class="capabilities-section">
                <div class="container">
                    <SectionHeader 
                        title="Our Mobile Development Capabilities" 
                        subtitle={Some("Comprehensive mobile solutions tailored to your business needs".to_string())}
                    />
                    
                    <div class="capabilities-grid">
                        {
                            mobile_capabilities.into_iter().map(|capability| {
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
            <section class="mobile-process-section">
                <div class="container">
                    <SectionHeader 
                        title="Our Mobile Development Process" 
                        subtitle={Some("How we deliver successful mobile applications".to_string())}
                    />
                    
                    <div class="process-timeline">
                        <div class="process-step">
                            <div class="process-step-number">{"01"}</div>
                            <div class="process-step-content">
                            <h3>{"Discovery & Strategy"}</h3>
                            <p>{"We begin by understanding your business goals, target users, and app requirements. Our team researches market trends, competitor apps, and user expectations to develop a comprehensive mobile strategy."}</p>
                        </div>
                    </div>
                    
                    <div class="process-step">
                        <div class="process-step-number">{"02"}</div>
                        <div class="process-step-content">
                            <h3>{"UX/UI Design"}</h3>
                            <p>{"Our designers create wireframes and interactive prototypes that visualize the user interface and experience. We conduct usability testing and refine designs based on feedback to ensure intuitive navigation and engaging interactions."}</p>
                        </div>
                    </div>
                    
                    <div class="process-step">
                        <div class="process-step-number">{"03"}</div>
                        <div class="process-step-content">
                            <h3>{"Development"}</h3>
                            <p>{"Our developers build your application using the most appropriate technologies for your requirements. We follow platform-specific guidelines and best practices to ensure high-quality code and optimal performance."}</p>
                        </div>
                    </div>
                    
                    <div class="process-step">
                        <div class="process-step-number">{"04"}</div>
                        <div class="process-step-content">
                            <h3>{"Testing & Quality Assurance"}</h3>
                            <p>{"We rigorously test your application across multiple devices, operating systems, and network conditions to ensure functionality, performance, security, and usability meet our high standards."}</p>
                        </div>
                    </div>
                    
                    <div class="process-step">
                        <div class="process-step-number">{"05"}</div>
                        <div class="process-step-content">
                            <h3>{"Deployment & Ongoing Support"}</h3>
                            <p>{"We handle the app store submission process and provide ongoing maintenance, updates, and support to ensure your application continues to evolve with user needs and platform changes."}</p>
                        </div>
                    </div>
                </div>
            </div>
        </section>
        
        // Case Studies Section
        <section class="case-studies-section">
            <div class="container">
                <SectionHeader 
                    title="Mobile App Success Stories" 
                    subtitle={Some("Real-world results from our mobile implementations".to_string())}
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
                    title="Our Mobile Technology Stack" 
                    subtitle={Some("Industry-leading technologies we leverage for mobile development".to_string())}
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
                    subtitle={Some("Common questions about our mobile development services".to_string())}
                />
                
                <div class="faq-grid">
                    <div class="faq-item">
                        <h3 class="faq-question">{"Should I build a native app or a cross-platform app?"}</h3>
                        <p class="faq-answer">{"The choice depends on your specific requirements. Native apps offer optimal performance and access to all platform features but require separate development for iOS and Android. Cross-platform solutions like Flutter and React Native provide cost efficiency and faster development with a single codebase that works on multiple platforms. During our consultation, we'll assess your needs and recommend the approach that best aligns with your goals, timeline, and budget."}</p>
                    </div>
                    
                    <div class="faq-item">
                        <h3 class="faq-question">{"How long does it take to develop a mobile app?"}</h3>
                        <p class="faq-answer">{"Development timelines vary based on complexity and scope. Simple apps with basic functionality might take 2-3 months, while complex applications with multiple features, integrations, and platforms can take 4-8 months or more. We follow an agile development approach, delivering functionality in phases to allow for earlier testing and feedback, which often results in better outcomes."}</p>
                    </div>
                    
                    <div class="faq-item">
                        <h3 class="faq-question">{"How do you ensure the security of mobile applications?"}</h3>
                        <p class="faq-answer">{"Security is integrated throughout our development process. We implement secure coding practices, data encryption for sensitive information, secure authentication mechanisms, and protection against common mobile vulnerabilities. We conduct security testing, including penetration testing for critical applications, and stay updated on platform-specific security requirements. For applications handling sensitive data, we implement additional security measures and compliance controls."}</p>
                    </div>
                    
                    <div class="faq-item">
                        <h3 class="faq-question">{"What ongoing support do you provide after app launch?"}</h3>
                        <p class="faq-answer">{"We offer comprehensive post-launch support including bug fixes, performance monitoring, regular updates to maintain compatibility with new OS versions, feature enhancements, and user feedback implementation. Our maintenance plans can be tailored to your needs, from basic support to full application management. We also provide analytics integration to help you understand user behavior and make data-driven decisions for future improvements."}</p>
                    </div>
                </div>
            </div>
        </section>
        
        // Call to Action
        <section class="cta-section">
            <div class="container">
                <div class="cta-content">
                    <h2>{"Ready to Build Your Mobile Application?"}</h2>
                    <p>{"Let's discuss how our mobile development expertise can help you engage users and achieve your business objectives."}</p>
                    <div class="cta-buttons">
                        <Link<Route> to={Route::Contact} classes="btn btn-primary">{"Schedule a Mobile Strategy Session"}</Link<Route>>
                        <Link<Route> to={Route::Projects} classes="btn btn-secondary">{"View Our Mobile App Portfolio"}</Link<Route>>
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