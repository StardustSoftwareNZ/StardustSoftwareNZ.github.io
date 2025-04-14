use crate::components::navigation::Navigation;
use crate::components::footer::Footer;
use crate::components::section_header::SectionHeader;
use crate::api::projects::get_projects;
use crate::components::project_card::ProjectCard;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum ProjectCategory {
    All,
    WebApplication,
    MobileApp,
    ArtificialIntelligence,
}

/// Projects - projects.rs
/// ======================
/// Projects showcase page for Stardust Software NZ
#[function_component(Projects)]
pub fn projects() -> Html {
    // State for the active category filter
    let active_category = use_state(|| ProjectCategory::All);
    
    // Get all projects
    let all_projects = get_projects();
    
    // Create featured projects (first 3)
    let featured_projects = all_projects.iter()
        .take(3)
        .cloned()
        .collect::<Vec<_>>();
    
    // Filter projects based on active category
    let filtered_projects = all_projects.iter()
        .skip(3) // Skip featured projects
        .filter(|project| {
            match *active_category {
                ProjectCategory::All => true,
                ProjectCategory::WebApplication => project.category == "Web Application",
                ProjectCategory::MobileApp => project.category == "Mobile Application",
                ProjectCategory::ArtificialIntelligence => project.category == "Artificial Intelligence",
            }
        })
        .cloned()
        .collect::<Vec<_>>();
    
    // Callback to handle category button clicks
    let on_category_click = {
        let active_category = active_category.clone();
        
        Callback::from(move |category: ProjectCategory| {
            active_category.set(category);
        })
    };
    
    // Helper function to determine if a category button is active
    let is_category_active = |category: &ProjectCategory| -> bool {
        *category == *active_category
    };

    html! {
        <div class="projects-page">
            // Navigation
            <Navigation 
                logo_url={"https://avatars.githubusercontent.com/u/80566578?s=200&v=4"} 
                company_name={"Stardust Software NZ"} 
            />
            
            // Page Header
            <section class="page-header">
                <div class="page-header-content">
                    <h1 class="page-title">{"Our Projects"}</h1>
                    <p class="page-subtitle">{"Discover our portfolio of innovative software solutions"}</p>
                </div>
                <div class="page-header-backdrop"></div>
            </section>
            
            // Featured Projects Section
            <section class="featured-projects-section">
                <div class="container">
                    <SectionHeader 
                        title="Featured Projects" 
                        subtitle={Some("Our most impactful work".to_string())}
                    />
                    
                    <div class="featured-projects-grid">
                        {
                            featured_projects.into_iter().map(|project| {
                                html! {
                                    <div class="featured-project-item">
                                        <ProjectCard key={project.id.clone()} project={project.clone()} />
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </section>
            
            // Project Categories Tabs
            <section class="project-categories-section">
                <div class="container">
                    <SectionHeader 
                        title="Browse by Category" 
                        subtitle={Some("Filter projects by expertise area".to_string())}
                    />
                    
                    <div class="category-tabs">
                        <button 
                            class={if is_category_active(&ProjectCategory::All) { "category-tab active" } else { "category-tab" }}
                            onclick={let cb = on_category_click.clone(); Callback::from(move |_| cb.emit(ProjectCategory::All))}
                        >
                            {"All"}
                        </button>
                        <button 
                            class={if is_category_active(&ProjectCategory::WebApplication) { "category-tab active" } else { "category-tab" }}
                            onclick={let cb = on_category_click.clone(); Callback::from(move |_| cb.emit(ProjectCategory::WebApplication))}
                        >
                            {"Web Applications"}
                        </button>
                        <button 
                            class={if is_category_active(&ProjectCategory::MobileApp) { "category-tab active" } else { "category-tab" }}
                            onclick={let cb = on_category_click.clone(); Callback::from(move |_| cb.emit(ProjectCategory::MobileApp))}
                        >
                            {"Mobile Apps"}
                        </button>
                        <button 
                            class={if is_category_active(&ProjectCategory::ArtificialIntelligence) { "category-tab active" } else { "category-tab" }}
                            onclick={let cb = on_category_click.clone(); Callback::from(move |_| cb.emit(ProjectCategory::ArtificialIntelligence))}
                        >
                            {"Artificial Intelligence"}
                        </button>
                    </div>
                    
                    <div class="projects-grid">
                        {
                            if filtered_projects.is_empty() {
                                html! {
                                    <div class="no-projects-message">
                                        <p>{"No projects found in this category."}</p>
                                        <p>{"Please check back later or browse another category."}</p>
                                    </div>
                                }
                            } else {
                                filtered_projects.into_iter().map(|project| {
                                    html! {
                                        <ProjectCard key={project.id.clone()} project={project.clone()} />
                                    }
                                }).collect::<Html>()
                            }
                        }
                    </div>
                </div>
            </section>
            
            // Project Process Section
            <section class="project-process-section">
                <div class="container">
                    <SectionHeader 
                        title="Our Project Approach" 
                        subtitle={Some("How we deliver successful projects".to_string())}
                    />
                    
                    <div class="process-steps">
                        <div class="process-step">
                            <div class="process-step-number">{"01"}</div>
                            <div class="process-step-content">
                                <h3>{"Discovery"}</h3>
                                <p>{"We begin by understanding your business objectives, user needs, and technical requirements through in-depth consultations."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"02"}</div>
                            <div class="process-step-content">
                                <h3>{"Design"}</h3>
                                <p>{"Our designers create intuitive interfaces and user experiences that are both visually appealing and highly functional."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"03"}</div>
                            <div class="process-step-content">
                                <h3>{"Development"}</h3>
                                <p>{"We build your solution using agile methodologies, with regular client check-ins and iterative improvements."}</p>
                            </div>
                        </div>
                        
                        <div class="process-step">
                            <div class="process-step-number">{"04"}</div>
                            <div class="process-step-content">
                                <h3>{"Delivery"}</h3>
                                <p>{"We launch your project with comprehensive testing, documentation, and training to ensure a smooth transition."}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            
            // Call to Action
            <section class="cta-section">
                <div class="container">
                    <div class="cta-content">
                        <h2>{"Ready to Build Something Amazing?"}</h2>
                        <p>{"Let's discuss how we can bring your project ideas to life."}</p>
                        <div class="cta-buttons">
                            <a href="/contact" class="btn btn-primary">{"Start a Project"}</a>
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