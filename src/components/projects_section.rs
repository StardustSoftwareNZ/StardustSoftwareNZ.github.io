use yew::prelude::*;
use crate::api::projects::{fetch_projects, get_hardcoded_projects};
use crate::components::section_header::SectionHeader;
use crate::components::project_card::ProjectCard;
use wasm_bindgen_futures::spawn_local;

#[derive(Properties, PartialEq)]
pub struct ProjectsSectionProps {
    #[prop_or(3)]
    pub limit: usize,
    #[prop_or_default]
    pub show_cta: bool,
    #[prop_or_default]
    pub id: Option<String>,
}

/// Projects section component to showcase featured projects
#[function_component(ProjectsSection)]
pub fn projects_section(props: &ProjectsSectionProps) -> Html {
    // State for storing projects
    let projects = use_state(|| Vec::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    
    // Fetch projects on component mount
    {
        let projects = projects.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    match fetch_projects().await {
                        Ok(fetched_projects) => {
                            projects.set(fetched_projects);
                            loading.set(false);
                        },
                        Err(err) => {
                            // Log the error
                            web_sys::console::error_1(&err);
                            // Fallback to hardcoded projects
                            projects.set(get_hardcoded_projects());
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
    
    // Limit projects based on the prop
    let display_projects = projects.iter()
        .take(props.limit)
        .cloned()
        .collect::<Vec<_>>();
    
    let id_attr = props.id.clone().unwrap_or("projects".to_string());
    
    html! {
        <section id={id_attr} class="projects-section">
            <div class="container">
                <SectionHeader
                    title="Featured Projects"
                    subtitle={Some("Some of our finest work".to_string())}
                />
                
                {
                    if *loading {
                        html! {
                            <div class="projects-loading-container">
                                <div class="loading-message">{"Loading our stellar projects"}</div>
                                <div class="loading-animation">
                                    <div class="bubble bubble-1"></div>
                                    <div class="bubble bubble-2"></div>
                                    <div class="bubble bubble-3"></div>
                                    <div class="loading-bar">
                                        <div class="loading-progress"></div>
                                    </div>
                                </div>
                            </div>
                        }
                    } else {
                        html! {
                            <>
                                {
                                    if let Some(err_msg) = &*error {
                                        html! {
                                            <div class="error-message">
                                                <p>{"Using fallback project data"}</p>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                
                                <div class="projects-grid">
                                    {
                                        display_projects.into_iter().map(|project| {
                                            html! {
                                                <ProjectCard key={project.id.clone()} project={project.clone()} />
                                            }
                                        }).collect::<Html>()
                                    }
                                </div>
                                
                                {
                                    if props.show_cta {
                                        html! {
                                            <div class="projects-cta">
                                                <a href="/projects" class="btn btn-outline">{"View All Projects"}</a>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </>
                        }
                    }
                }
            </div>
        </section>
    }
}