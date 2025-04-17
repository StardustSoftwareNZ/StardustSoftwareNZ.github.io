use crate::components::navigation::Navigation;
use crate::components::footer::Footer;
use crate::components::section_header::SectionHeader;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::FocusEvent;

/// Cloud Readiness Assessment - cloud_readiness_assessment.rs
/// ===================
/// Cloud Readiness Assessment page for Stardust Software NZ
#[function_component(CloudReadinessAssessment)]
pub fn cloud_readiness_assessment() -> Html {
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
            title: "Infrastructure & Architecture".to_string(),
            description: "Evaluate your current IT infrastructure and its compatibility with cloud environments.".to_string(),
            icon: "☁️".to_string(),
        },
        AssessmentDimension {
            title: "Data Migration Strategy".to_string(),
            description: "Assess your plan for migrating data to the cloud, including security and compliance.".to_string(),
            icon: "💾".to_string(),
        },
        AssessmentDimension {
            title: "Security & Compliance".to_string(),
            description: "Determine your understanding and implementation of cloud security best practices and regulatory requirements.".to_string(),
            icon: "🔒".to_string(),
        },
        AssessmentDimension {
            title: "Skills & Expertise".to_string(),
            description: "Gauge your team's current cloud computing skills and identify any knowledge gaps.".to_string(),
            icon: "🧑‍💻".to_string(),
        },
        AssessmentDimension {
            title: "Cost Management & Optimization".to_string(),
            description: "Review your strategies for managing and optimizing cloud spending.".to_string(),
            icon: "💰".to_string(),
        },
        AssessmentDimension {
            title: "Governance & Policies".to_string(),
            description: "Evaluate your organization's policies and governance framework for cloud adoption and usage.".to_string(),
            icon: "⚙️".to_string(),
        },
    ];

    // Sample assessment questions
    let assessment_questions = vec![
        AssessmentQuestion {
            question: "How would you describe your current IT infrastructure?".to_string(),
            dimension: "Infrastructure & Architecture".to_string(),
            options: vec![
                "Primarily on-premises with limited virtualization".to_string(),
                "Significant virtualization but limited cloud presence".to_string(),
                "Hybrid environment with some cloud services".to_string(),
                "Cloud-first strategy with a significant cloud footprint".to_string(),
            ],
        },
        AssessmentQuestion {
            question: "What is your organization's strategy for migrating data to the cloud?".to_string(),
            dimension: "Data Migration Strategy".to_string(),
            options: vec![
                "No formal data migration strategy in place".to_string(),
                "Planning a lift-and-shift approach for most data".to_string(),
                "Considering re-platforming or refactoring applications and data".to_string(),
                "Well-defined strategy with clear timelines and security measures".to_string(),
            ],
        },
        AssessmentQuestion {
            question: "How confident are you in your organization's cloud security posture?".to_string(),
            dimension: "Security & Compliance".to_string(),
            options: vec![
                "Limited understanding of cloud security best practices".to_string(),
                "Basic cloud security measures implemented".to_string(),
                "Actively implementing and monitoring advanced cloud security controls".to_string(),
                "Strong security focus with dedicated cloud security expertise and compliance adherence".to_string(),
            ],
        },
        // Additional questions would be added here
    ];

    // Benefits of assessment
    let benefits = vec![
        Benefit {
            title: "Clear Migration Path".to_string(),
            description: "Gain a structured understanding of the steps required for a successful cloud migration.".to_string(),
            icon: "🛤️".to_string(),
        },
        Benefit {
            title: "Reduced Risks".to_string(),
            description: "Identify potential pitfalls and challenges before they impact your cloud adoption journey.".to_string(),
            icon: "⚠️".to_string(),
        },
        Benefit {
            title: "Optimized Spending".to_string(),
            description: "Develop strategies for efficient cloud resource utilization and cost management.".to_string(),
            icon: "📉".to_string(),
        },
        Benefit {
            title: "Enhanced Security".to_string(),
            description: "Strengthen your cloud security framework and ensure compliance with relevant regulations.".to_string(),
            icon: "🛡️".to_string(),
        },
    ];

    // Assessment process stages
    let assessment_stages = vec![
        AssessmentStage {
            title: "Initial Questionnaire".to_string(),
            description: "Complete our online questionnaire to assess your current cloud readiness across key dimensions.".to_string(),
            icon: "📝".to_string(),
        },
        AssessmentStage {
            title: "Expert Review".to_string(),
            description: "Our cloud experts analyze your responses and identify your strengths and areas for improvement.".to_string(),
            icon: "🧐".to_string(),
        },
        AssessmentStage {
            title: "Customized Report".to_string(),
            description: "Receive a detailed report outlining your cloud readiness score and specific recommendations.".to_string(),
            icon: "📊".to_string(),
        },
        AssessmentStage {
            title: "Strategy Workshop".to_string(),
            description: "Participate in a collaborative workshop to develop a tailored cloud adoption strategy and roadmap.".to_string(),
            icon: "🤝".to_string(),
        },
    ];

    // Testimonials
    let testimonials = vec![
        Testimonial {
            quote: "The cloud readiness assessment helped us understand the critical areas we needed to address before moving our core applications to the cloud. Their recommendations were invaluable.".to_string(),
            author: "David Miller".to_string(),
            company: "TechSolutions Inc.".to_string(),
            position: "CIO".to_string(),
        },
        Testimonial {
            quote: "Stardust Software's assessment process was thorough and insightful. We now have a clear roadmap and are much more confident in our cloud migration plans.".to_string(),
            author: "Emily White".to_string(),
            company: "Global Retail Group".to_string(),
            position: "Head of IT".to_string(),
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
        <div class="cloud-readiness-page">
            // Navigation
            <Navigation
                logo_url={"https://avatars.githubusercontent.com/u/80566578?s=200&v=4"}
                company_name={"Stardust Software NZ"}
            />

            // Hero Section
            <section class="service-hero">
                <div class="service-hero-content">
                    <h1 class="service-hero-title">{"Cloud Readiness Assessment"}</h1>
                    <p class="service-hero-subtitle">{"Determine your organization's readiness for a successful cloud journey"}</p>
                </div>
                <div class="service-hero-backdrop"></div>
            </section>

            // Overview Section
            <section class="service-overview-section">
                <div class="container">
                    <div class="service-overview-content">
                        <div class="service-overview-text">
                            <h2>{"Is Your Business Ready for the Cloud?"}</h2>
                            <p>{"Migrating to the cloud offers numerous benefits, but it's crucial to ensure your organization is truly prepared. Our comprehensive Cloud Readiness Assessment evaluates key aspects of your infrastructure, security, skills, and strategy to help you make informed decisions and plan for a smooth transition."}</p>
                            <p>{"This assessment examines six vital dimensions of cloud readiness, providing you with actionable insights and a tailored roadmap to leverage the power of the cloud effectively."}</p>
                        </div>
                        <div class="service-overview-image">
                            <img src="https://www.publicdomainpictures.net/pictures/90000/velka/blue-sky-with-cloud-1397245823liX.jpg" alt="Cloud Readiness" />
                        </div>
                    </div>
                </div>
            </section>

            // Assessment Dimensions Section
            <section class="dimensions-section">
                <div class="container">
                    <SectionHeader
                        title="Assessment Dimensions"
                        subtitle={Some("Our comprehensive evaluation covers six critical areas of cloud readiness".to_string())}
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
                        subtitle={Some("Understand the advantages of conducting a cloud readiness assessment".to_string())}
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
                        subtitle={Some("How our cloud readiness evaluation works".to_string())}
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
                        subtitle={Some("Get a glimpse of the questions covered in our evaluation".to_string())}
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
                        title="Start Your Cloud Readiness Assessment"
                        subtitle={Some("Fill out the form below to begin your cloud journey".to_string())}
                    />
                    <form
                        class="assessment-form"
                        onsubmit={on_submit}
                        action="https://formspree.io/f/your_formspree_endpoint" // Replace with your Formspree endpoint
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
                                <option value="other">{"Other"}</option>
                            </select>
                        </div>

                        <div class="form-group full-width">
                            <label for="company_size">{"Company Size"}</label>
                            <select id="company_size" name="company_size" required={true}>
                                <option value="">{"Select company size"}</option>
                                <option value="1-50">{"1-50 employees"}</option>
                                <option value="51-200">{"51-200 employees"}</option>
                                <option value="201-500">{"201-500 employees"}</option>
                                <option value="500+">{"500+ employees"}</option>
                            </select>
                        </div>

                        <div class="form-group full-width">
                            <label for="cloud_experience">{"Current Cloud Experience"}</label>
                            <select id="cloud_experience" name="cloud_experience" required={true}>
                                <option value="">{"Select your experience level"}</option>
                                <option value="none">{"No cloud experience"}</option>
                                <option value="exploring">{"Exploring cloud options"}</option>
                                <option value="some">{"Using some cloud services"}</option>
                                <option value="significant">{"Significant cloud usage"}</option>
                            </select>
                        </div>

                        <div class="form-group full-width">
                            <label for="cloud_goals">{"Primary Goals for Cloud Adoption"}</label>
                            <textarea id="cloud_goals" name="cloud_goals" rows="4" required={true}></textarea>
                        </div>

                        <div class="form-group full-width">
                            <label class="checkbox-label">
                                <input type="checkbox" name="consent" required={true} />
                                <span>{"I agree to receive communications regarding my assessment and cloud adoption strategies."}</span>
                            </label>
                        </div>

                        <div class="form-submit">
                            <button type="submit" class="btn btn-primary">{"Start Cloud Assessment"}</button>
                        </div>
                    </form>
                </div>
            </section>

            // FAQ Section
            <section class="faq-section">
                <div class="container">
                    <SectionHeader
                        title="Frequently Asked Questions"
                        subtitle={Some("Common questions about our cloud readiness assessment".to_string())}
                    />

                    <div class="faq-grid">
                        <div class="faq-item">
                            <h3 class="faq-question">{"How long does the cloud readiness assessment take?"}</h3>
                            <p class="faq-answer">{"The initial online questionnaire typically takes around 20-30 minutes to complete. Our expert review and customized report generation usually take 3-5 business days. The optional strategy workshop is a 2-hour session."}</p>
                        </div>

                        <div class="faq-item">
                            <h3 class="faq-question">{"Who should participate in the cloud readiness assessment?"}</h3>
                            <p class="faq-answer">{"We recommend involving key stakeholders from your IT department, security team, and business leadership to ensure a comprehensive understanding of your organization's cloud readiness."}</p>
                        </div>

                        <div class="faq-item">
                            <h3 class="faq-question">{"Is this assessment suitable for all types of businesses?"}</h3>
                            <p class="faq-answer">{"Yes, our cloud readiness assessment is designed to be adaptable for businesses of all sizes and across various industries. We tailor our analysis and recommendations to your specific needs and goals."}</p>
                        </div>

                        <div class="faq-item">
                            <h3 class="faq-question">{"What kind of recommendations will we receive?"}</h3>
                            <p class="faq-answer">{"You will receive a detailed report with specific recommendations covering infrastructure adjustments, data migration strategies, security enhancements, skill development, cost optimization, and governance framework improvements."}</p>
                        </div>
                    </div>
                </div>
            </section>

            // Call to Action
            <section class="cta-section">
                <div class="container">
                    <div class="cta-content">
                        <h2>{"Ready to Embrace the Cloud with Confidence?"}</h2>
                        <p>{"Take the first step towards a successful cloud adoption journey. Contact us for a free consultation to discuss your cloud readiness assessment."}</p>
                        <div class="cta-buttons">
                            <Link<Route> to={Route::Contact} classes="btn btn-primary">{"Schedule a Free Consultation"}</Link<Route>>
                            <Link<Route> to={Route::CloudService} classes="btn btn-secondary">{"Explore Our Cloud Services"}</Link<Route>>
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