use crate::components::navigation::Navigation;
use crate::components::footer::Footer;
use crate::components::section_header::SectionHeader;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

/// Cloud Solutions Service - cloud_solutions_service.rs
/// ===================
/// Cloud Solutions Services page for Stardust Software NZ
#[function_component(CloudService)]
pub fn cloud_solutions_service() -> Html {
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

    // Define Cloud capabilities
    #[derive(Clone)]
    struct CloudCapability {
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
            title: "Modernizing Academic Research Data Management for NZODN".to_string(),
            industry: "Oceanographic Research, Environmental Science, Academic Research".to_string(),
            challenge: "The New Zealand Ocean Data Network (NZODN) required a solution to publish academic research data on their data portal as part of contract work for the National Institute of Water and Atmospheric Research (NIWA). The existing data migration process was in disarray - undocumented, written in bespoke Python and Bash scripts by a third party, poorly maintained, and incapable of handling continuous ingestion streams. This created significant barriers for researchers trying to publish their findings and made data management unnecessarily complex and error-prone.".to_string(),
            solution: "We designed and implemented a robust, maintainable data migration system that prioritized security, automation, and sustainability. The solution utilized Secure File Transfer Protocol (SFTP) for safe and reliable file transfers, standardized Bash scripts for processing, and scheduled Cron jobs for automated execution. We integrated GeoServer to process and publish tile layers to a Web Map Service (WMS) on the NZODN portal, while GeoNetwork was employed to manage and publish comprehensive metadata about the academic research data. The entire system was thoroughly documented with clear process flows, error handling procedures, and maintenance guidelines.".to_string(),
            result: "The modernized data migration system dramatically improved the efficiency and reliability of publishing academic research data to the NZODN portal. Researchers now have a clear, documented process for submitting their data, with automated validation checks ensuring data quality. The system can handle continuous data ingestion streams, eliminating previous bottlenecks. Administrative overhead has been reduced by approximately 70%, while successful data publications increased by 45%. The improved metadata management through GeoNetwork has enhanced the discoverability and usability of research data, leading to increased citations and research impact. NIWA now has a sustainable, maintainable system that can evolve with changing research needs.".to_string(),
            image_url: "https://upload.wikimedia.org/wikipedia/commons/1/10/Rotoiti_Bathymetry.png".to_string(),
        },
        CaseStudy {
            title: "Wellington Chamber of Commerce Website Modernization".to_string(),
            industry: "Business Association, Non-Profit, Professional Services".to_string(),
            challenge: "The Wellington Chamber of Commerce and Business Central needed to modernize their outdated website while preserving years of valuable content. A significant portion of their resources and publications existed only as PDF documents, creating accessibility issues and limiting their digital reach. The client needed to maintain their existing Content Management System (CMS) while transforming hundreds of PDF-based resources into modern, responsive web content. Manual conversion would have been prohibitively time-consuming and inconsistent, potentially disrupting service to their business members.".to_string(),
            solution: "We developed a custom PDF-to-web conversion pipeline that seamlessly integrated with their existing CMS. The solution utilized specialized Bash scripts and Markdown parsing utilities to automate the extraction and transformation of PDF content into clean, semantic HTML. Each document was processed to ensure proper heading structure, responsive images, and accessibility compliance. The conversion pipeline included validation checks to maintain content integrity and metadata preservation to ensure searchability. This tooling worked alongside their existing CMS, allowing staff to continue using familiar workflows while dramatically improving the quality and accessibility of the content.".to_string(),
            result: "The custom conversion utilities accelerated the content migration process by a factor of ten compared to manual methods, allowing the entire library of resources to be transformed efficiently. The Chamber now delivers content through progressive, responsive web pages that load faster and are accessible across all devices - a significant improvement over the previous PDF-only approach. Search engine visibility improved dramatically as content became indexable, increasing organic traffic by 35%. Member engagement metrics showed a 28% increase in resource usage following the migration. The Chamber staff now have a standardized process for converting new publications to web format, ensuring sustainable content management going forward.".to_string(),
            image_url: "https://www.newzealandchambers.co.nz/media/5645386/wellingtonchamber10august2016.jpg?width=684".to_string(),
        },
    ];

    // Define Cloud capabilities
    let cloud_capabilities = vec![
        CloudCapability {
            title: "Geospatial Information Systems".to_string(),
            description: "Utilize advanced geospatial technologies to analyze, visualize, and manage location-based data for better decision-making.".to_string(),
            icon: "🗺️".to_string(),
        },
        CloudCapability {
            title: "Cloud Migration".to_string(),
            description: "Seamlessly transition your on-premises infrastructure to secure, scalable cloud environments with minimal disruption to your operations.".to_string(),
            icon: "☁️".to_string(),
        },
        CloudCapability {
            title: "Infrastructure as Code".to_string(),
            description: "Automate infrastructure provisioning and management using code-based approaches for consistent, repeatable, and version-controlled deployments.".to_string(),
            icon: "📝".to_string(),
        },
        CloudCapability {
            title: "Serverless Architecture".to_string(),
            description: "Build modern applications using event-driven, serverless computing models that scale automatically and optimize costs.".to_string(),
            icon: "⚡".to_string(),
        },
        CloudCapability {
            title: "Containerization".to_string(),
            description: "Package applications and dependencies into lightweight, portable containers for consistent deployment across environments.".to_string(),
            icon: "📦".to_string(),
        },
        CloudCapability {
            title: "Cloud Security".to_string(),
            description: "Implement comprehensive security controls, compliance frameworks, and best practices to protect your data and applications in the cloud.".to_string(),
            icon: "🔒".to_string(),
        },
        CloudCapability {
            title: "DevOps Automation".to_string(),
            description: "Streamline development and operations with CI/CD pipelines, monitoring, and automated deployment processes.".to_string(),
            icon: "🔄".to_string(),
        },
    ];

    // Define technology stack
    let technologies = vec![
        Technology {
            name: "PostgreSQL".to_string(),
            description: "Open-source relational database management system".to_string(),
            image: "https://www.postgresql.org/media/img/about/press/elephant.png".to_string(),
        },
        Technology {
            name: "GeoServer".to_string(),
            description: "Open-source server for sharing geospatial data".to_string(),
            image: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRTuXrnd87ishV1fCmY69X6Cc7J6oDPBEy-Fg&s".to_string(),
        },
        Technology {
            name: "GeoNetwork".to_string(),
            description: "Open-source catalog application for geospatial data".to_string(),
            image: "https://camptocamp.com/expertise/partner-and-products/geonetwork/image-thumb__1163__hero-header-slider-without-teaser/geonetwork_logo-header.webp".to_string(),
        },
        Technology {
            name: "Supabase".to_string(),
            description: "Open-source Firebase alternative for building applications".to_string(),
            image: "https://logowik.com/content/uploads/images/supabase-icon9119.logowik.com.webp".to_string(),
        },
        Technology {
            name: "Firebase".to_string(),
            description: "Google's mobile platform for building apps".to_string(),
            image: "https://brandlogos.net/wp-content/uploads/2025/03/firebase_icon-logo_brandlogos.net_tcvck.png".to_string(),
        },
        Technology {
            name: "AWS".to_string(),
            description: "Comprehensive cloud computing platform".to_string(),
            image: "https://upload.wikimedia.org/wikipedia/commons/thumb/9/93/Amazon_Web_Services_Logo.svg/1280px-Amazon_Web_Services_Logo.svg.png".to_string(),
        },
        Technology {
            name: "Google Cloud".to_string(),
            description: "Suite of cloud computing services by Google".to_string(),
            image: "https://cloud.google.com/_static/cloud/images/social-icon-google-cloud-1200-630.png".to_string(),
        },
        Technology {
            name: "Microsoft Azure".to_string(),
            description: "Enterprise-grade cloud computing platform".to_string(),
            image: "https://upload.wikimedia.org/wikipedia/commons/thumb/f/fa/Microsoft_Azure.svg/1200px-Microsoft_Azure.svg.png".to_string(),
        },
        Technology {
            name: "Kubernetes".to_string(),
            description: "Container orchestration and management".to_string(),
            image: "https://upload.wikimedia.org/wikipedia/commons/thumb/3/39/Kubernetes_logo_without_workmark.svg/1200px-Kubernetes_logo_without_workmark.svg.png".to_string(),
        },
        Technology {
            name: "Terraform".to_string(),
            description: "Infrastructure as code automation".to_string(),
            image: "https://www.datocms-assets.com/2885/1620155116-brandhcterraformverticalcolor.svg".to_string(),
        },
        Technology {
            name: "Docker".to_string(),
            description: "Containerization platform for applications".to_string(),
            image: "https://www.docker.com/wp-content/uploads/2022/03/vertical-logo-monochromatic.png".to_string(),
        },
    ];

    html! {
        <div class="cloud-service-page">
            // Navigation
            <Navigation 
                logo_url={"https://avatars.githubusercontent.com/u/80566578?s=200&v=4"} 
                company_name={"Stardust Software NZ"} 
            />
            
            // Hero Section
            <section class="service-hero">
                <div class="service-hero-content">
                    <h1 class="service-hero-title">{"Cloud Solutions"}</h1>
                    <p class="service-hero-subtitle">{"Scalable, secure, and reliable cloud infrastructure for your business"}</p>
                </div>
                <div class="service-hero-backdrop"></div>
            </section>
            
            // Overview Section
            <section class="service-overview-section">
                <div class="container">
                    <div class="service-overview-content">
                        <div class="service-overview-text">
                            <h2>{"Harness the Power of the Cloud"}</h2>
                            <p>{"At Stardust Software, we help businesses leverage cloud technologies to transform their operations, enhance scalability, and reduce infrastructure costs. Our cloud solutions are designed to meet your specific business needs, whether you're looking to migrate existing applications, build new cloud-native systems, or optimize your current cloud environment."}</p>
                            <p>{"We understand that every organization has unique requirements and challenges. Our team of cloud architects and engineers work closely with you to design and implement solutions that align with your business goals, security requirements, and budget constraints."}</p>
                            <p>{"From infrastructure design and migration to ongoing management and optimization, our comprehensive cloud services ensure you get the most value from your cloud investments while maintaining security, performance, and reliability."}</p>
                        </div>
                        <div class="service-overview-image">
                            <img src="https://images.unsplash.com/photo-1544197150-b99a580bb7a8?ixlib=rb-1.2.1&auto=format&fit=crop&w=1350&q=80" alt="Cloud Infrastructure" />
                        </div>
                    </div>
                </div>
            </section>
            
            // Capabilities Section
            <section class="capabilities-section">
                <div class="container">
                    <SectionHeader 
                        title="Our Cloud Capabilities" 
                        subtitle={Some("Comprehensive cloud solutions tailored to your business needs".to_string())}
                    />
                    
                    <div class="capabilities-grid">
                        {
                            cloud_capabilities.into_iter().map(|capability| {
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
            <section class="cloud-process-section">
                <div class="container">
                    <SectionHeader 
                        title="Our Cloud Implementation Process" 
                        subtitle={Some("How we deliver successful cloud projects".to_string())}
                    />
                    
                    <div class="process-timeline">
                        <div class="process-step">
                            <div class="process-step-number">{"01"}</div>
                            <div class="process-step-content">
                                <h3>{"Assessment & Strategy"}</h3>
                                <p>{"We begin by understanding your current infrastructure, business requirements, and goals. Our team conducts a thorough assessment to identify migration candidates, security needs, and potential optimization opportunities."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"02"}</div>
                            <div class="process-step-content">
                                <h3>{"Architecture Design"}</h3>
                                <p>{"Based on the assessment, we design a cloud architecture that meets your performance, security, compliance, and cost requirements. This includes selecting the right cloud providers, services, and deployment models."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"03"}</div>
                            <div class="process-step-content">
                                <h3>{"Migration & Implementation"}</h3>
                                <p>{"We execute the migration or implementation using industry best practices, minimizing disruption to your business operations. Our approach includes thorough testing and validation at each stage."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"04"}</div>
                            <div class="process-step-content">
                                <h3>{"Automation & DevOps"}</h3>
                                <p>{"We implement infrastructure as code, CI/CD pipelines, and automated testing to streamline development and operations, ensuring consistent and reliable deployments."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"05"}</div>
                            <div class="process-step-content">
                                <h3>{"Monitoring & Optimization"}</h3>
                                <p>{"We establish comprehensive monitoring and alerting systems, and continuously optimize your cloud environment for performance, security, and cost-efficiency."}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            
            // Case Studies Section
            <section class="case-studies-section">
                <div class="container">
                    <SectionHeader 
                        title="Cloud Success Stories" 
                        subtitle={Some("Real-world results from our cloud implementations".to_string())}
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
                        title="Our Cloud Technology Stack" 
                        subtitle={Some("Industry-leading technologies we leverage for cloud solutions".to_string())}
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
                        subtitle={Some("Common questions about our cloud services".to_string())}
                    />
                    
                    <div class="faq-grid">
                        <div class="faq-item">
                            <h3 class="faq-question">{"How secure is cloud computing for my business?"}</h3>
                            <p class="faq-answer">{"When implemented correctly, cloud environments can be more secure than traditional on-premises infrastructure. We employ industry best practices for cloud security, including encryption, identity and access management, network security, and continuous monitoring. Our team stays updated on the latest security threats and compliance requirements to ensure your data and applications remain protected."}</p>
                        </div>
                        
                        <div class="faq-item">
                            <h3 class="faq-question">{"How long does a typical cloud migration take?"}</h3>
                            <p class="faq-answer">{"The timeline varies depending on the complexity and scale of your infrastructure. Simple migrations might take 1-2 months, while enterprise-level migrations can take 6-12 months or more. We use a phased approach to minimize disruption and deliver value incrementally. During our assessment phase, we'll provide a detailed timeline based on your specific requirements."}</p>
                        </div>
                        
                        <div class="faq-item">
                            <h3 class="faq-question">{"How do you handle compliance requirements in the cloud?"}</h3>
                            <p class="faq-answer">{"We have experience implementing cloud solutions that meet various compliance standards including GDPR, HIPAA, PCI DSS, and ISO 27001. Our approach includes selecting compliant cloud providers, implementing required security controls, establishing audit trails and monitoring, and documenting compliance evidence. We work closely with your compliance team to ensure all requirements are met."}</p>
                        </div>
                        
                        <div class="faq-item">
                            <h3 class="faq-question">{"How do you optimize cloud costs?"}</h3>
                            <p class="faq-answer">{"Our cost optimization strategy includes right-sizing resources, implementing auto-scaling, utilizing reserved instances or savings plans, optimizing storage usage, and identifying and removing unused resources. We establish continuous monitoring of cloud spend and regularly review your environment to identify new optimization opportunities. Our goal is to ensure you get maximum value from your cloud investment without overspending."}</p>
                        </div>
                    </div>
                </div>
            </section>
            
            // Call to Action
            <section class="cta-section">
                <div class="container">
                    <div class="cta-content">
                        <h2>{"Ready to Transform Your Infrastructure with Cloud?"}</h2>
                        <p>{"Let's discuss how our cloud solutions can help you improve scalability, reduce costs, and drive innovation."}</p>
                        <div class="cta-buttons">
                            <Link<Route> to={Route::Contact} classes="btn btn-primary">{"Schedule a Cloud Consultation"}</Link<Route>>
                            <Link<Route> to={Route::CloudReadinessAssessment} classes="btn btn-secondary">{"Get a Free Cloud Readiness Assessment"}</Link<Route>>
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