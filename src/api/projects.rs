use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub category: String,
    pub description: String,
    pub image_url: String,
    pub case_study_url: String,
}

pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            id: "project-1".to_string(),
            title: "Portfolio Website".to_string(),
            category: "Web Application".to_string(),
            description: "A portfolio website for an academic, made with Deno Fresh, Postgresql and Supabase.".to_string(),
            image_url: "https://images.unsplash.com/photo-1460925895917-afdab827c52f?ixlib=rb-1.2.1&auto=format&fit=crop&w=500&q=60".to_string(),
            case_study_url: "https://woodrock.deno.dev/".to_string(),
        },
        Project {
            id: "project-2".to_string(),
            title: "Wellington Bus Timetable".to_string(),
            category: "Web Application".to_string(),
            description: "A bus timetable for the Wellington Region, made with Deno Fresh and Metlink API.".to_string(),
            image_url: "https://images.unsplash.com/photo-1526498460520-4c246339dccb?ixlib=rb-1.2.1&auto=format&fit=crop&w=500&q=60".to_string(),
            case_study_url: "https://bus-timetable.deno.dev/".to_string(),
        },
        Project {
            id: "project-3".to_string(),
            title: "Renewed Roots".to_string(),
            category: "Web Application".to_string(),
            description: "An online shop for selling recycled furniture, made with Deno Fresh".to_string(),
            image_url: "https://images.unsplash.com/photo-1504868584819-f8e8b4b6d7e3?ixlib=rb-1.2.1&auto=format&fit=crop&w=500&q=60".to_string(),
            case_study_url: "https://flogging-furniture.deno.dev/".to_string(),
        },
        Project {
            id: "project-4".to_string(),
            title: "Ionic Scholar".to_string(),
            category: "Mobile Application".to_string(),
            description: "A citations tracker for academics made with Ionic React Framework.".to_string(),
            image_url: "https://images.unsplash.com/photo-1556742049-0cfed4f6a45d?ixlib=rb-1.2.1&auto=format&fit=crop&w=500&q=60".to_string(),
            case_study_url: "https://github.com/woodRock/ionic-scholar".to_string(),
        },
    ]
}