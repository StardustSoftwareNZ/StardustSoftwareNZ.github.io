use crate::components::navigation::Navigation;
use crate::components::footer::Footer;
use crate::components::section_header::SectionHeader;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::FocusEvent;
use wasm_bindgen::JsCast;
use web_sys::HtmlFormElement;

/// AI Readiness Assessment - ai_readiness_assessment.rs
/// ===================
/// AI Readiness Assessment page for Stardust Software NZ
#[function_component(AiReadinessAssessment)]
pub fn ai_readiness_assessment() -> Html {
    // Define assessment dimensions with descriptions
    #[derive(Clone)]
    struct AssessmentDimension {
        title: String,
        description: String,
        icon: String,
    }

    // Define assessment questions
    #[derive(Clone)]
    struct AssessmentQuestion {
        question: String,
        dimension: String,
        options: Vec<String>,
    }

    // Define benefits
    #[derive(Clone)]
    struct Benefit {
        title: String,
        description: String,
        icon: String,
    }

    // Define assessment stages
    #[derive(Clone)]
    struct AssessmentStage {
        title: String,
        description: String,
        icon: String,
    }

    // Define testimonials
    #[derive(Clone)]
    struct Testimonial {
        quote: String,
        author: String,
        company: String,
        position: String,
    }

    // Assessment dimensions
    let assessment_dimensions = vec![
        AssessmentDimension {
            title: "Data Readiness".to_string(),
            description: "Evaluate the quality, quantity, and accessibility of your organization's data assets.".to_string(),
            icon: "📊".to_string(),
        },
        AssessmentDimension {
            title: "Technical Infrastructure".to_string(),
            description: "Assess your current IT systems, cloud capabilities, and technical architecture.".to_string(),
            icon: "🖥️".to_string(),
        },
        AssessmentDimension {
            title: "Business Strategy".to_string(),
            description: "Determine how well AI initiatives align with your overall business objectives.".to_string(),
            icon: "🎯".to_string(),
        },
        AssessmentDimension {
            title: "Organizational Culture".to_string(),
            description: "Gauge your team's openness to adopting data-driven decision making and new technologies.".to_string(),
            icon: "👥".to_string(),
        },
        AssessmentDimension {
            title: "Skills & Expertise".to_string(),
            description: "Evaluate your team's current AI capabilities and identify skills gaps.".to_string(),
            icon: "🧠".to_string(),
        },
        AssessmentDimension {
            title: "Governance & Ethics".to_string(),
            description: "Review your processes for responsible AI development and data governance.".to_string(),
            icon: "⚖️".to_string(),
        },
    ];

    // Sample assessment questions
    let assessment_questions = vec![
        AssessmentQuestion {
            question: "How would you describe your organization's data collection and storage practices?".to_string(),
            dimension: "Data Readiness".to_string(),
            options: vec![
                "We have no formal data collection process".to_string(),
                "We collect data but it's scattered across different systems".to_string(),
                "We have centralized data storage but quality issues exist".to_string(),
                "We have well-structured, high-quality data with proper governance".to_string(),
            ],
        },
        AssessmentQuestion {
            question: "How does your organization currently use data for decision making?".to_string(),
            dimension: "Business Strategy".to_string(),
            options: vec![
                "Decisions are primarily based on intuition or experience".to_string(),
                "We use basic reporting but not for strategic decisions".to_string(),
                "We regularly use data analytics for some business decisions".to_string(),
                "Data-driven decision making is deeply embedded in our culture".to_string(),
            ],
        },
        AssessmentQuestion {
            question: "Does your organization have computing resources suitable for AI workloads?".to_string(),
            dimension: "Technical Infrastructure".to_string(),
            options: vec![
                "We have limited computing resources".to_string(),
                "We have standard business computing resources only".to_string(),
                "We have some cloud or high-performance computing capabilities".to_string(),
                "We have dedicated infrastructure for data science and AI".to_string(),
            ],
        },
        // Additional questions would be added here
    ];

    // Benefits of assessment
    let benefits = vec![
        Benefit {
            title: "Strategic Roadmap".to_string(),
            description: "Receive a tailored AI implementation roadmap based on your organization's unique position and priorities.".to_string(),
            icon: "🗺️".to_string(),
        },
        Benefit {
            title: "Risk Mitigation".to_string(),
            description: "Identify potential challenges and obstacles before they impact your AI initiatives.".to_string(),
            icon: "🛡️".to_string(),
        },
        Benefit {
            title: "Resource Optimization".to_string(),
            description: "Focus your investments and efforts on the most impactful AI opportunities for your business.".to_string(),
            icon: "💰".to_string(),
        },
        Benefit {
            title: "Competitive Insights".to_string(),
            description: "Benchmark your AI readiness against industry standards and competitors.".to_string(),
            icon: "📈".to_string(),
        },
    ];

    // Assessment process stages
    let assessment_stages = vec![
        AssessmentStage {
            title: "Initial Assessment".to_string(),
            description: "Complete our comprehensive online assessment covering all dimensions of AI readiness.".to_string(),
            icon: "✅".to_string(),
        },
        AssessmentStage {
            title: "Expert Analysis".to_string(),
            description: "Our AI specialists analyze your responses and identify key strengths and improvement areas.".to_string(),
            icon: "🔍".to_string(),
        },
        AssessmentStage {
            title: "Detailed Report".to_string(),
            description: "Receive a customized report with scores across all dimensions and specific recommendations.".to_string(),
            icon: "📝".to_string(),
        },
        AssessmentStage {
            title: "Strategy Session".to_string(),
            description: "Engage in a facilitated workshop to develop your AI implementation roadmap.".to_string(),
            icon: "💡".to_string(),
        },
    ];

    // Testimonials
    let testimonials = vec![
        Testimonial {
            quote: "The AI readiness assessment provided invaluable insights that helped us prioritize our data infrastructure improvements before diving into advanced AI projects.".to_string(),
            author: "Sarah Johnson".to_string(),
            company: "OceanFresh Seafoods".to_string(),
            position: "CTO".to_string(),
        },
        Testimonial {
            quote: "Stardust's assessment framework identified critical gaps in our governance processes that we wouldn't have caught until much later. They saved us from potential compliance issues.".to_string(),
            author: "Michael Chen".to_string(),
            company: "PacificHealth Networks".to_string(),
            position: "Director of Innovation".to_string(),
        },
    ];

    // State for tracking form submission status
    let submitted = use_state(|| false);

    // Create a function to handle form submission
    let on_submit = {
        let submitted = submitted.clone();
        
        Callback::from(move |e: FocusEvent| {
            // Don't prevent default here - let the form submit naturally
            // Just update the UI state
            submitted.set(true);
        })
    };
    
    html! {
        <div class="ai-readiness-page">
            // Navigation
            <Navigation 
                logo_url={"https://avatars.githubusercontent.com/u/80566578?s=200&v=4"} 
                company_name={"Stardust Software NZ"} 
            />
            
            // Hero Section
            <section class="service-hero">
                <div class="service-hero-content">
                    <h1 class="service-hero-title">{"AI Readiness Assessment"}</h1>
                    <p class="service-hero-subtitle">{"Evaluate your organization's preparedness for AI implementation"}</p>
                </div>
                <div class="service-hero-backdrop"></div>
            </section>
            
            // Overview Section
            <section class="service-overview-section">
                <div class="container">
                    <div class="service-overview-content">
                        <div class="service-overview-text">
                            <h2>{"Is Your Organization Ready for AI?"}</h2>
                            <p>{"Successful AI implementation requires more than just cutting-edge technology. It demands the right data, infrastructure, skills, and organizational culture. Our comprehensive AI Readiness Assessment helps you evaluate your current capabilities and identifies the specific steps needed to prepare your organization for successful AI adoption."}</p>
                            <p>{"This assessment examines six critical dimensions of AI readiness, providing you with actionable insights and a customized roadmap for implementing AI solutions that deliver real business value."}</p>
                        </div>
                        <div class="service-overview-image">
                            <img src="https://images.unsplash.com/photo-1551288049-bebda4e38f71?ixlib=rb-1.2.1&auto=format&fit=crop&w=1350&q=80" alt="AI Readiness" />
                        </div>
                    </div>
                </div>
            </section>
            
            // Assessment Dimensions Section
            <section class="dimensions-section">
                <div class="container">
                    <SectionHeader 
                        title="Assessment Dimensions" 
                        subtitle={Some("Our comprehensive evaluation covers six critical areas of AI readiness".to_string())}
                    />
                    
                    <div class="dimensions-grid">
                        {
                            assessment_dimensions.into_iter().map(|dimension| {
                                html! {
                                    <div class="dimension-card">
                                        <div class="dimension-icon">{dimension.icon}</div>
                                        <h3 class="dimension-title">{dimension.title}</h3>
                                        <p class="dimension-description">{dimension.description}</p>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </section>
            
            // Benefits Section
            <section class="benefits-section">
                <div class="container">
                    <SectionHeader 
                        title="Benefits of Our Assessment" 
                        subtitle={Some("Why conducting an AI readiness assessment is critical for successful implementation".to_string())}
                    />
                    
                    <div class="benefits-grid">
                        {
                            benefits.into_iter().map(|benefit| {
                                html! {
                                    <div class="benefit-card">
                                        <div class="benefit-icon">{benefit.icon}</div>
                                        <h3 class="benefit-title">{benefit.title}</h3>
                                        <p class="benefit-description">{benefit.description}</p>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </section>
            
            // Assessment Process Section
            <section class="process-section">
                <div class="container">
                    <SectionHeader 
                        title="Assessment Process" 
                        subtitle={Some("How our AI readiness evaluation works".to_string())}
                    />
                    
                    <div class="process-timeline">
                        {
                            assessment_stages.into_iter().enumerate().map(|(index, stage)| {
                                let step_number = format!("{:02}", index + 1);
                                html! {
                                    <div class="process-step">
                                        <div class="process-step-number">{step_number}</div>
                                        <div class="process-step-content">
                                            <h3>{stage.title}</h3>
                                            <p>{stage.description}</p>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </section>
            
            // Sample Questions Section
            <section class="questions-section">
                <div class="container">
                    <SectionHeader 
                        title="Sample Assessment Questions" 
                        subtitle={Some("Preview of our comprehensive evaluation framework".to_string())}
                    />
                    
                    <div class="questions-grid">
                        {
                            assessment_questions.into_iter().map(|question| {
                                html! {
                                    <div class="question-card">
                                        <div class="question-dimension">{question.dimension}</div>
                                        <h3 class="question-text">{question.question}</h3>
                                        <div class="question-options">
                                            {
                                                question.options.into_iter().map(|option| {
                                                    html! {
                                                        <div class="option-item">
                                                            <input type="radio" disabled={true} />
                                                            <span>{option}</span>
                                                        </div>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </section>
            
            // Get Started Form Section
            <section class="assessment-form-section">
                <div class="container">
                    <SectionHeader 
                        title="Start Your Assessment" 
                        subtitle={Some("Fill out the form below to begin your AI readiness journey".to_string())}
                    />       
                    <form 
                        class="assessment-form" 
                        onsubmit={on_submit} 
                        action="https://formspree.io/f/mpwpynqy" 
                        method="POST"
                    >
                        <div class="form-group">
                            <label for="name">{"Full Name"}</label>
                            <input type="text" id="name" name="name" required={true} />
                        </div>
                        
                        <div class="form-group">
                            <label for="email">{"Business Email"}</label>
                            <input type="email" id="email" name="email" required={true} />
                        </div>
                        
                        <div class="form-group">
                            <label for="company">{"Company Name"}</label>
                            <input type="text" id="company" name="company" required={true} />
                        </div>
                        
                        <div class="form-group">
                            <label for="position">{"Your Position"}</label>
                            <input type="text" id="position" name="position" required={true} />
                        </div>
                        
                        <div class="form-group full-width">
                            <label for="industry">{"Industry"}</label>
                            <select id="industry" name="industry" required={true}>
                                <option value="">{"Select your industry"}</option>
                                <option value="healthcare">{"Healthcare"}</option>
                                <option value="finance">{"Finance & Banking"}</option>
                                <option value="retail">{"Retail & E-commerce"}</option>
                                <option value="manufacturing">{"Manufacturing"}</option>
                                <option value="technology">{"Technology"}</option>
                                <option value="agriculture">{"Agriculture"}</option>
                                <option value="education">{"Education"}</option>
                                <option value="energy">{"Energy & Utilities"}</option>
                                <option value="transportation">{"Transportation & Logistics"}</option>
                                <option value="other">{"Other"}</option>
                            </select>
                        </div>
                        
                        <div class="form-group full-width">
                            <label for="company_size">{"Company Size"}</label>
                            <select id="company_size" name="company_size" required={true}>
                                <option value="">{"Select company size"}</option>
                                <option value="1-10">{"1-10 employees"}</option>
                                <option value="11-50">{"11-50 employees"}</option>
                                <option value="51-200">{"51-200 employees"}</option>
                                <option value="201-500">{"201-500 employees"}</option>
                                <option value="501-1000">{"501-1000 employees"}</option>
                                <option value="1000+">{"1000+ employees"}</option>
                            </select>
                        </div>
                        
                        <div class="form-group full-width">
                            <label for="ai_experience">{"Current AI Experience"}</label>
                            <select id="ai_experience" name="ai_experience" required={true}>
                                <option value="">{"Select your experience level"}</option>
                                <option value="none">{"No experience with AI"}</option>
                                <option value="exploring">{"Exploring AI possibilities"}</option>
                                <option value="planning">{"Planning first AI project"}</option>
                                <option value="implemented">{"Have implemented basic AI"}</option>
                                <option value="advanced">{"Advanced AI implementation"}</option>
                            </select>
                        </div>
                        
                        <div class="form-group full-width">
                            <label for="goals">{"Primary Goals for AI Implementation"}</label>
                            <textarea id="goals" name="goals" rows="4" required={true}></textarea>
                        </div>
                        
                        <div class="form-group full-width">
                            <label class="checkbox-label">
                                <input type="checkbox" name="consent" required={true} />
                                <span>{"I agree to receive communications regarding my assessment and AI implementation strategies."}</span>
                            </label>
                        </div>
                        
                        <div class="form-submit">
                            <button type="submit" class="btn btn-primary">{"Start My Assessment"}</button>
                        </div>
                    </form>
                </div>
            </section>
            
            // FAQ Section
            <section class="faq-section">
                <div class="container">
                    <SectionHeader 
                        title="Frequently Asked Questions" 
                        subtitle={Some("Common questions about our AI readiness assessment".to_string())}
                    />
                    
                    <div class="faq-grid">
                        <div class="faq-item">
                            <h3 class="faq-question">{"How long does the assessment process take?"}</h3>
                            <p class="faq-answer">{"The online portion of the assessment takes approximately 30-45 minutes to complete. The expert analysis and preparation of your custom report typically requires 5-7 business days. The strategy session is a 2-3 hour workshop scheduled at your convenience."}</p>
                        </div>
                        
                        <div class="faq-item">
                            <h3 class="faq-question">{"Who from our organization should participate in the assessment?"}</h3>
                            <p class="faq-answer">{"For best results, we recommend involving stakeholders from IT, data management, business operations, and executive leadership. This cross-functional approach ensures we capture a complete picture of your organization's readiness."}</p>
                        </div>
                        
                        <div class="faq-item">
                            <h3 class="faq-question">{"Is this assessment suitable for small businesses?"}</h3>
                            <p class="faq-answer">{"Yes, we've designed our assessment to accommodate organizations of all sizes. For smaller businesses, we focus on pragmatic, scalable approaches to AI implementation that align with your resources and growth objectives."}</p>
                        </div>
                        
                        <div class="faq-item">
                            <h3 class="faq-question">{"Do we need technical staff to complete the assessment?"}</h3>
                            <p class="faq-answer">{"While input from technical team members is valuable, our assessment is designed to be accessible to business leaders without deep technical expertise. We're happy to assist with any technical aspects during the process."}</p>
                        </div>
                    </div>
                </div>
            </section>
            
            // Call to Action
            <section class="cta-section">
                <div class="container">
                    <div class="cta-content">
                        <h2>{"Ready to Start Your AI Journey?"}</h2>
                        <p>{"Book a free 30-minute consultation with our AI specialists to learn more about the assessment process."}</p>
                        <div class="cta-buttons">
                            <Link<Route> to={Route::Contact} classes="btn btn-primary">{"Schedule Consultation"}</Link<Route>>
                            <Link<Route> to={Route::AiService} classes="btn btn-secondary">{"Learn About Our AI Services"}</Link<Route>>
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