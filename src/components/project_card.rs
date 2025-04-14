use yew::prelude::*;
use crate::api::projects::Project;

#[derive(Properties, PartialEq)]
pub struct ProjectCardProps {
    pub project: Project,
}

/// Project card component to display featured projects
#[function_component(ProjectCard)]
pub fn project_card(props: &ProjectCardProps) -> Html {
    let project = &props.project;
    let image_style = format!("background-image: url('{}')", project.image_url);
    
    html! {
        <div class="project-card">
            <div class="project-image" style={image_style}></div>
            <div class="project-info">
                <h3 class="project-title">{ &project.title }</h3>
                <p class="project-category">{ &project.category }</p>
                <p class="project-description">{ &project.description }</p>
                <a href={project.case_study_url.clone()} class="project-link">{"View Case Study"}</a>
            </div>
        </div>
    }
}