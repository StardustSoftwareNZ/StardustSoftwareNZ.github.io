use crate::components::navigation::Navigation;
use crate::components::footer::Footer;
use crate::components::section_header::SectionHeader;
use yew::prelude::*;

/// AI Service - ai_service.rs
/// ===================
/// AI Services page for Stardust Software NZ
#[function_component(AiService)]
pub fn ai_service() -> Html {
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

    // Define AI capabilities
    #[derive(Clone)]
    struct AiCapability {
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
            title: "Predictive Maintenance for Manufacturing".to_string(),
            industry: "Manufacturing".to_string(),
            challenge: "A large manufacturing company struggled with unexpected equipment failures causing costly downtime and production delays.".to_string(),
            solution: "We developed an AI-driven predictive maintenance system that analyzes sensor data to forecast potential equipment failures before they occur.".to_string(),
            result: "Reduced unplanned downtime by 78%, maintenance costs by 35%, and extended equipment lifespan by an average of 3 years.".to_string(),
            image_url: "https://images.unsplash.com/photo-1581092921461-eab62e97a780?ixlib=rb-1.2.1&auto=format&fit=crop&w=1350&q=80".to_string(),
        },
        CaseStudy {
            title: "Customer Service Chatbot for e-Commerce".to_string(),
            industry: "Retail".to_string(),
            challenge: "An e-commerce company was experiencing high customer service costs and struggling to provide 24/7 support.".to_string(),
            solution: "We built a natural language processing (NLP) chatbot that handles common customer inquiries, product recommendations, and troubleshooting.".to_string(),
            result: "90% of customer queries now resolved without human intervention, 65% reduction in customer service costs, and 24/7 customer support availability.".to_string(),
            image_url: "https://images.unsplash.com/photo-1568952433726-3896e3881c65?ixlib=rb-1.2.1&auto=format&fit=crop&w=1350&q=80".to_string(),
        },
    ];

    // Define AI capabilities
    let ai_capabilities = vec![
        AiCapability {
            title: "Machine Learning".to_string(),
            description: "Build custom machine learning models trained on your business data to identify patterns and make predictions that drive business value.".to_string(),
            icon: "🧠".to_string(),
        },
        AiCapability {
            title: "Natural Language Processing".to_string(),
            description: "Implement advanced NLP to analyze text data, extract insights from documents, and create conversational interfaces.".to_string(),
            icon: "💬".to_string(),
        },
        AiCapability {
            title: "Computer Vision".to_string(),
            description: "Develop image and video recognition systems for object detection, classification, and visual inspection automation.".to_string(),
            icon: "👁️".to_string(),
        },
        AiCapability {
            title: "Predictive Analytics".to_string(),
            description: "Forecast business trends, customer behavior, and operational outcomes using historical data and statistical algorithms.".to_string(),
            icon: "📈".to_string(),
        },
        AiCapability {
            title: "Recommendation Systems".to_string(),
            description: "Create personalized recommendation engines to improve customer engagement, increase sales, and enhance user experiences.".to_string(),
            icon: "🎯".to_string(),
        },
        AiCapability {
            title: "Process Automation".to_string(),
            description: "Automate complex business processes using intelligent decision-making systems and robotic process automation (RPA).".to_string(),
            icon: "⚙️".to_string(),
        },
    ];

    // Define technology stack
    let technologies = vec![
        Technology {
            name: "Python".to_string(),
            description: "Oeneral purpose programming language.".to_string(),
            image: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcR1z0LC70CF3lPE1Xe-Uka4Y5sSlLzrAVHCQg&s".to_string(),
        },
        Technology {
            name: "PyTorch".to_string(),
            description: "Deep learning platform for research and production".to_string(),
            image: "https://pytorch.org/assets/images/pytorch-logo.png".to_string(),
        },
        Technology {
            name: "scikit-learn".to_string(),
            description: "Machine learning library for classical algorithms".to_string(),
            image: "https://scikit-learn.org/stable/_static/scikit-learn-logo-small.png".to_string(),
        },
        Technology {
            name: "Hugging Face".to_string(),
            description: "State-of-the-art NLP models and tools".to_string(),
            image: "https://huggingface.co/front/assets/huggingface_logo.svg".to_string(),
        },
    ];

    html! {
        <div class="ai-service-page">
            // Navigation
            <Navigation 
                logo_url={"https://avatars.githubusercontent.com/u/80566578?s=200&v=4"} 
                company_name={"Stardust Software NZ"} 
            />
            
            // Hero Section
            <section class="service-hero">
                <div class="service-hero-content">
                    <h1 class="service-hero-title">{"Artificial Intelligence Solutions"}</h1>
                    <p class="service-hero-subtitle">{"Harnessing the power of AI to solve complex business problems"}</p>
                </div>
                <div class="service-hero-backdrop"></div>
            </section>
            
            // Overview Section
            <section class="service-overview-section">
                <div class="container">
                    <div class="service-overview-content">
                        <div class="service-overview-text">
                            <h2>{"Transform Your Business With AI"}</h2>
                            <p>{"At Stardust Software, we specialize in developing custom artificial intelligence solutions that drive business value. Our AI services help organizations automate processes, gain deeper insights from their data, and create intelligent applications that adapt and improve over time."}</p>
                            <p>{"We understand that implementing AI successfully requires both technical expertise and a deep understanding of your business domain. Our team of data scientists, machine learning engineers, and industry experts work closely with you to identify high-value opportunities and deliver tailored AI solutions that address your specific challenges."}</p>
                            <p>{"Whether you're looking to optimize operations, enhance customer experiences, or gain a competitive edge, our AI services can help you unlock new possibilities and drive innovation in your business."}</p>
                        </div>
                        <div class="service-overview-image">
                            <img src="https://images.unsplash.com/photo-1580894894513-541e068a3e2b?ixlib=rb-1.2.1&auto=format&fit=crop&w=1350&q=80" alt="AI Development" />
                        </div>
                    </div>
                </div>
            </section>
            
            // Capabilities Section
            <section class="capabilities-section">
                <div class="container">
                    <SectionHeader 
                        title="Our AI Capabilities" 
                        subtitle={Some("Comprehensive AI solutions tailored to your business needs".to_string())}
                    />
                    
                    <div class="capabilities-grid">
                        {
                            ai_capabilities.into_iter().map(|capability| {
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
            <section class="ai-process-section">
                <div class="container">
                    <SectionHeader 
                        title="Our AI Development Process" 
                        subtitle={Some("How we deliver successful AI projects".to_string())}
                    />
                    
                    <div class="process-timeline">
                        <div class="process-step">
                            <div class="process-step-number">{"01"}</div>
                            <div class="process-step-content">
                                <h3>{"Discovery & Assessment"}</h3>
                                <p>{"We begin by understanding your business objectives, data assets, and potential AI use cases. Our team conducts a thorough assessment to identify opportunities with the highest ROI."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"02"}</div>
                            <div class="process-step-content">
                                <h3>{"Data Strategy"}</h3>
                                <p>{"We help you collect, clean, and organize your data to ensure it's ready for AI applications. This may include data warehousing, integration, and governance recommendations."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"03"}</div>
                            <div class="process-step-content">
                                <h3>{"AI Model Development"}</h3>
                                <p>{"Our data scientists develop and train custom AI models using your data. We iterate through multiple approaches to find the best solution for your specific needs."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"04"}</div>
                            <div class="process-step-content">
                                <h3>{"Integration & Deployment"}</h3>
                                <p>{"We seamlessly integrate AI models into your existing systems and workflows, ensuring they provide value in real-world applications."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"05"}</div>
                            <div class="process-step-content">
                                <h3>{"Monitoring & Optimization"}</h3>
                                <p>{"We implement continuous monitoring to ensure your AI solutions maintain accuracy and performance over time, with regular updates and improvements."}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            
            // Case Studies Section
            <section class="case-studies-section">
                <div class="container">
                    <SectionHeader 
                        title="AI Success Stories" 
                        subtitle={Some("Real-world results from our AI implementations".to_string())}
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
                        title="Our AI Technology Stack" 
                        subtitle={Some("Industry-leading technologies we leverage for AI development".to_string())}
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
                        subtitle={Some("Common questions about our AI services".to_string())}
                    />
                    
                    <div class="faq-grid">
                        <div class="faq-item">
                            <h3 class="faq-question">{"Do we need a large amount of data to implement AI?"}</h3>
                            <p class="faq-answer">{"While having more data generally leads to better AI models, we can still create valuable solutions with limited data using techniques like transfer learning and synthetic data generation. During our initial assessment, we'll evaluate your current data assets and provide recommendations."}</p>
                        </div>
                        
                        <div class="faq-item">
                            <h3 class="faq-question">{"How long does an AI project typically take?"}</h3>
                            <p class="faq-answer">{"The timeline varies depending on complexity and scope. Simple proof-of-concept projects can be completed in 4-8 weeks, while more complex enterprise implementations might take 3-6 months. We follow an agile approach, delivering value incrementally throughout the project."}</p>
                        </div>
                        
                        <div class="faq-item">
                            <h3 class="faq-question">{"How do you ensure AI models are ethical and unbiased?"}</h3>
                            <p class="faq-answer">{"We follow rigorous practices to test for and mitigate bias in our AI models. This includes diverse training data, bias detection tools, regular audits, and transparency in our model development process. We also prioritize explainability, allowing users to understand how AI decisions are made."}</p>
                        </div>
                        
                        <div class="faq-item">
                            <h3 class="faq-question">{"What ongoing support do you provide after deployment?"}</h3>
                            <p class="faq-answer">{"We offer comprehensive support packages that include model monitoring, performance optimization, retraining as needed, and technical support. Our team also provides knowledge transfer and training to help your team understand and maintain the AI solutions we build."}</p>
                        </div>
                    </div>
                </div>
            </section>
            
            // Call to Action
            <section class="cta-section">
                <div class="container">
                    <div class="cta-content">
                        <h2>{"Ready to Transform Your Business with AI?"}</h2>
                        <p>{"Let's discuss how our AI solutions can help you solve complex problems and create competitive advantages."}</p>
                        <div class="cta-buttons">
                            <a href="/contact" class="btn btn-primary">{"Schedule a Consultation"}</a>
                            <a href="/ai-assessment" class="btn btn-secondary">{"Take Our AI Readiness Assessment"}</a>
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