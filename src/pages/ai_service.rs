use crate::components::navigation::Navigation;
use crate::components::footer::Footer;
use crate::components::section_header::SectionHeader;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

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
            title: "Automated Fish Classification Using Unprocessed Fatty Acid Chromatographic Data: A Machine Learning Approach".to_string(),
            industry: "Fish Processing, Chemistry, Artificial Intelligence".to_string(),
            challenge: "Only about 40% of a fish is edible fillet, with the remaining 60% being suitable for either low-value fertilizer or high-value omega-3 concentrate production. However, the viability of high-value processing depends on the biochemical composition of the biomass, which varies with species, tissue type, and season. Determining biomass quality requires fatty acid profiling through Gas Chromatography—a precise but time-consuming process that relies heavily on expert interpretation.".to_string(),
            solution: "This paper explores the use of classification and feature selection algorithms to automate the interpretation of Gas Chromatography data. It investigates whether models such as Support Vector Machines (SVMs) can accurately classify marine biomass based on its fatty acid composition and identifies key features that contribute to classification performance.".to_string(),
            result: "The experiments showed that SVMs can effectively classify compositionally diverse marine biomass using raw chromatographic data. Visualizations from the SVMs aid interpretability by highlighting important features. Moreover, incorporating feature selection significantly reduced dimensionality, improved accuracy, and accelerated processing—boosting classification speed by up to 4× on high-dimensional, low-sample-size datasets.".to_string(),
            image_url: "https://s3.animalia.bio/animals/photos/full/original/2560pxannual-report-of-the-commissioners-of-fisheries-game-and-forests-of-the-state-of-new-york-18961900-18740990974.webp".to_string(),
        },
        CaseStudy {
            title: "Hook, Line and Spectra: Machine Learning for Fish Species and Part Classification using Rapid Evaporative Ionization Mass Spectrometry".to_string(),
            industry: "Fish Processing, Chemistry, Artificial Intelligence".to_string(),
            challenge: "Marine biomass composition analysis is traditionally labor-intensive and reliant on domain-specific expertise. Standard methods are time-consuming and not scalable for real-time or high-throughput applications in industry. There is a growing need for rapid, accurate, and interpretable techniques for assessing biomass quality across varied species and tissues.".to_string(),
            solution: "This study explores the use of Rapid Evaporative Ionization Mass Spectrometry (REIMS) combined with advanced machine learning—particularly deep learning and unsupervised pre-training of transformer models—for automated biomass composition analysis. Diverse biochemical profiles, modeled through fish species and body parts, served as test cases. Additionally, the study applies explainability methods such as LIME and Grad-CAM to interpret decision-making in high-performing classifiers.".to_string(),
            result: "Deep learning methods consistently outperformed traditional machine learning across all tasks. The pretrained transformer achieved 99.62% accuracy in fish species classification and 84.06% accuracy for fish body part classification. Explainability techniques successfully identified critical features driving model predictions, making REIMS combined with machine learning a powerful and interpretable approach for marine biomass analysis. This method holds significant promise for industrial quality control, product optimization, and food safety monitoring.".to_string(),
            image_url: "https://s3.animalia.bio/animals/photos/full/original/fmib-41562-common-mackerel-scomber-scombrusjpeg.webp".to_string(),
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
            name: "Pandas".to_string(),
            description: "Data manipulation and analysis".to_string(),
            image: "https://cdn.worldvectorlogo.com/logos/pandas.svg".to_string(),
        },
        Technology {
            name: "Numpy".to_string(),
            description: "Multi-dimensional arrays and matrices".to_string(),
            image: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQRC2C9EVtvZjW_wQ3f9bEP2Fgla230C3kVYQ&s".to_string(),
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
                            <Link<Route> to={Route::Contact} classes="btn btn-primary">{"Schedule a Consultation"}</Link<Route>>
                            <Link<Route> to={Route::NotFound} classes="btn btn-secondary">{"Take Our AI Readiness Assessment"}</Link<Route>>
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