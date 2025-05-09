use crate::components::navigation::Navigation;
use crate::components::footer::Footer;
use crate::components::section_header::SectionHeader;
use crate::components::process_section::ProcessSection;
use crate::api::projects::{get_projects, fetch_projects, get_hardcoded_projects};
use crate::components::project_card::ProjectCard;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Debug, PartialEq)]
enum ProjectCategory {
    All,
    WebApplication,
    MobileApp,
    ArtificialIntelligence,
    CloudComputing,
    CustomSoftware,
}

/// Projects - projects.rs
/// ======================
/// Projects showcase page for Stardust Software NZ
#[function_component(Projects)]
pub fn projects() -> Html {
    // State for the active category filter
    let active_category = use_state(|| ProjectCategory::All);
    
    // State for storing projects
    let projects_data = use_state(|| get_hardcoded_projects());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    
    // Fetch projects on component mount
    {
        let projects_data = projects_data.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    match fetch_projects().await {
                        Ok(fetched_projects) => {
                            projects_data.set(fetched_projects);
                            loading.set(false);
                        },
                        Err(err) => {
                            // Log the error
                            web_sys::console::error_1(&err);
                            // Fallback to hardcoded projects
                            projects_data.set(get_hardcoded_projects());
                            error.set(Some(format!("Error: {:?}", err)));
                            loading.set(false);
                        }
                    }
                });
                
                || ()
            },
            (), // Empty dependencies - only run once on mount
        );
    }
    
    // Create featured projects (first 3)
    let featured_projects = projects_data.iter()
        .take(3)
        .cloned()
        .collect::<Vec<_>>();
    
    // Filter projects based on active category
    let filtered_projects = projects_data.iter()
        .skip(3) // Skip featured projects
        .filter(|project| {
            match *active_category {
                ProjectCategory::All => true,
                ProjectCategory::WebApplication => project.category == "Web Application",
                ProjectCategory::MobileApp => project.category == "Mobile Application",
                ProjectCategory::ArtificialIntelligence => project.category == "Artificial Intelligence",
                ProjectCategory::CloudComputing => project.category == "Cloud Computing",
                ProjectCategory::CustomSoftware => project.category == "Custom Software",
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
            
            // Loading State
            if *loading {
                <section class="loading-section">
                    <div class="container">
                        <div class="projects-loading-container">
                            <div class="loading-message">{"Discovering stellar projects"}</div>
                            <div class="loading-animation">
                                <div class="orbit">
                                    <div class="planet"></div>
                                    <div class="satellite satellite-1"></div>
                                    <div class="satellite satellite-2"></div>
                                    <div class="satellite satellite-3"></div>
                                </div>
                                <div class="loading-bar">
                                    <div class="loading-progress"></div>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>
            } else {
                <>
                    // Error Message (if any)
                    if let Some(err_msg) = &*error {
                        <section class="error-section">
                            <div class="container">
                                <div class="error-message">
                                    <p>{format!("Note: Using fallback data. {}", err_msg)}</p>
                                </div>
                            </div>
                        </section>
                    }
                    
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
                                <button 
                                    class={if is_category_active(&ProjectCategory::CloudComputing) { "category-tab active" } else { "category-tab" }}
                                    onclick={let cb = on_category_click.clone(); Callback::from(move |_| cb.emit(ProjectCategory::CloudComputing))}
                                >
                                    {"Cloud Computing"}
                                </button>
                                <button 
                                    class={if is_category_active(&ProjectCategory::CustomSoftware) { "category-tab active" } else { "category-tab" }}
                                    onclick={let cb = on_category_click.clone(); Callback::from(move |_| cb.emit(ProjectCategory::CustomSoftware))}
                                >
                                    {"Custom Software"}
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
                </>
            }
            
            // Project Process Section
            <ProcessSection id={"process"} />
            
            // Call to Action
            <section class="cta-section">
                <div class="container">
                    <div class="cta-content">
                        <h2>{"Ready to Build Something Amazing?"}</h2>
                        <p>{"Let's discuss how we can bring your project ideas to life."}</p>
                        <div class="cta-buttons">
                            <Link<Route> to={Route::Contact} classes="btn btn-primary">{"Start a Project"}</Link<Route>>
                            <Link<Route> to={Route::NotFound} classes="btn btn-secondary">{"Explore Our Services"}</Link<Route>>
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