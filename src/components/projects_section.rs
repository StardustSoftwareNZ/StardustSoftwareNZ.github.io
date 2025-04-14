use yew::prelude::*;
use crate::api::projects::get_projects;
use crate::components::section_header::SectionHeader;
use crate::components::project_card::ProjectCard;

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
    // Get all projects and limit them based on the prop
    let all_projects = get_projects();
    let projects = all_projects.into_iter().take(props.limit).collect::<Vec<_>>();
    
    let id_attr = props.id.clone().unwrap_or("projects".to_string());
    
    html! {
        <section id={id_attr} class="projects-section">
            <div class="container">
                <SectionHeader 
                    title="Featured Projects" 
                    subtitle={Some("Some of our finest work".to_string())}
                />
                
                <div class="projects-grid">
                    {
                        projects.into_iter().map(|project| {
                            html! {
                                <ProjectCard key={project.id.clone()} project={project.clone()} />
                            }
                        }).collect::<Html>()
                    }
                </div>
                
                if props.show_cta {
                    <div class="projects-cta">
                        <a href="/projects" class="btn btn-outline">{"View All Projects"}</a>
                    </div>
                }
            </div>
        </section>
    }
}