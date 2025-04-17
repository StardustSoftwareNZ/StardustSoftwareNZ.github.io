use crate::components::navigation::Navigation;
use crate::components::footer::Footer;
use crate::components::section_header::SectionHeader;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::FocusEvent;

/// Free Custom Software Needs Assessment - free_needs_assessment.rs
/// ===================
/// Free Needs Assessment page for Custom Software by Stardust Software NZ
#[function_component(FreeNeedsAssessment)]
pub fn free_needs_assessment() -> Html {
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

    // Benefits of the free assessment
    let benefits = vec![
        Benefit {
            title: "Clarity on Requirements".to_string(),
            description: "Gain a clear understanding of your software needs and objectives.".to_string(),
            icon: "💡".to_string(),
        },
        Benefit {
            title: "Identify Potential Solutions".to_string(),
            description: "Explore different custom software approaches that can address your challenges.".to_string(),
            icon: "🔍".to_string(),
        },
        Benefit {
            title: "Understand Potential Costs".to_string(),
            description: "Get a preliminary understanding of the investment involved in custom software development.".to_string(),
            icon: "💰".to_string(),
        },
        Benefit {
            title: "Expert Consultation".to_string(),
            description: "Benefit from the insights of our experienced software development team.".to_string(),
            icon: "🧑‍💼".to_string(),
        },
    ];

    // Assessment process stages
    let assessment_stages = vec![
        AssessmentStage {
            title: "Submit Your Inquiry".to_string(),
            description: "Fill out our simple form with details about your project idea and needs.".to_string(),
            icon: "✍️".to_string(),
        },
        AssessmentStage {
            title: "Initial Consultation".to_string(),
            description: "We'll schedule a free 30-minute call to discuss your requirements in detail.".to_string(),
            icon: "📞".to_string(),
        },
        AssessmentStage {
            title: "Needs Analysis".to_string(),
            description: "Our team will analyze your input and consultation notes to understand your core needs.".to_string(),
            icon: "🧐".to_string(),
        },
        AssessmentStage {
            title: "Preliminary Recommendations".to_string(),
            description: "Receive a summary of our initial recommendations and potential next steps.".to_string(),
            icon: "📝".to_string(),
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
        <div class="free-needs-assessment-page">
            // Navigation
            <Navigation
                logo_url={"https://avatars.githubusercontent.com/u/80566578?s=200&v=4"}
                company_name={"Stardust Software NZ"}
            />

            // Hero Section
            <section class="service-hero">
                <div class="service-hero-content">
                    <h1 class="service-hero-title">{"Free Custom Software Needs Assessment"}</h1>
                    <p class="service-hero-subtitle">{"Let us help you define your custom software project and its potential"}</p>
                </div>
                <div class="service-hero-backdrop"></div>
            </section>

            // Overview Section
            <section class="service-overview-section">
                <div class="container">
                    <div class="service-overview-content">
                        <div class="service-overview-text">
                            <h2>{"Unlock Your Software Potential with a Free Assessment"}</h2>
                            <p>{"Are you considering custom software to streamline your business processes, improve efficiency, or create a unique product? Our free needs assessment is the perfect starting point. We'll work with you to understand your challenges, define your requirements, and explore the possibilities of tailored software solutions – all at no cost to you."}</p>
                            <p>{"This complimentary assessment will provide you with valuable insights and help you make informed decisions about your software development journey."}</p>
                        </div>
                        <div class="service-overview-image">
                            <img src="https://www.publicdomainpictures.net/pictures/90000/velka/blue-sky-with-cloud-1397245823liX.jpg" alt="Custom Software Development" />
                        </div>
                    </div>
                </div>
            </section>

            // Benefits Section
            <section class="benefits-section">
                <div class="container">
                    <SectionHeader
                        title="Benefits of Our Free Assessment"
                        subtitle={Some("Discover the advantages of starting your custom software journey with us".to_string())}
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
                        title="Our Simple Assessment Process"
                        subtitle={Some("How we'll help you understand your custom software needs".to_string())}
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

            // Get Started Form Section
            <section class="assessment-form-section">
                <div class="container">
                    <SectionHeader
                        title="Request Your Free Needs Assessment"
                        subtitle={Some("Tell us about your project and let's start the conversation".to_string())}
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
                            <input type="text" id="company" name="company" />
                        </div>

                        <div class="form-group">
                            <label for="phone">{"Phone Number"}</label>
                            <input type="tel" id="phone" name="phone" />
                        </div>

                        <div class="form-group full-width">
                            <label for="project_description">{"Brief Project Description"}</label>
                            <textarea id="project_description" name="project_description" rows="5" required={true} placeholder="Please describe your project idea, key objectives, and any specific challenges you're facing."></textarea>
                        </div>

                        <div class="form-group full-width">
                            <label for="current_systems">{"Current Systems (if applicable)"}</label>
                            <textarea id="current_systems" name="current_systems" rows="3" placeholder="What systems are you currently using that this new software might integrate with or replace?"></textarea>
                        </div>

                        <div class="form-group full-width">
                            <label for="budget">{"Approximate Budget (optional)"}</label>
                            <select id="budget" name="budget">
                                <option value="">{"Select Budget Range (Optional)"}</option>
                                <option value="less_than_10k">{"Less than $10,000"}</option>
                                <option value="10k_to_25k">{"$10,000 - $25,000"}</option>
                                <option value="25k_to_50k">{"$25,000 - $50,000"}</option>
                                <option value="50k_to_100k">{"$50,000 - $100,000"}</option>
                                <option value="more_than_100k">{"More than $100,000"}</option>
                                <option value="not_sure">{"Not Sure"}</option>
                            </select>
                        </div>

                        <div class="form-group full-width">
                            <label class="checkbox-label">
                                <input type="checkbox" name="consent" required={true} />
                                <span>{"I agree to be contacted by Stardust Software NZ regarding my needs assessment."}</span>
                            </label>
                        </div>

                        <div class="form-submit">
                            <button type="submit" class="btn btn-primary">{"Request Free Assessment"}</button>
                        </div>
                    </form>
                </div>
            </section>

            // Call to Action
            <section class="cta-section">
                <div class="container">
                    <div class="cta-content">
                        <h2>{"Ready to Discuss Your Custom Software Vision?"}</h2>
                        <p>{"Take advantage of our free, no-obligation needs assessment. Let's explore how custom software can transform your business."}</p>
                        <div class="cta-buttons">
                            <Link<Route> to={Route::Contact} classes="btn btn-primary">{"Contact Us for More Info"}</Link<Route>>
                            <Link<Route> to={Route::CustomService} classes="btn btn-secondary">{"Learn About Our Custom Software Services"}</Link<Route>>
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