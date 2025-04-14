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
            title: "Enterprise Dashboard".to_string(),
            category: "Web Application".to_string(),
            description: "A comprehensive analytics dashboard for a financial services company.".to_string(),
            image_url: "https://images.unsplash.com/photo-1460925895917-afdab827c52f?ixlib=rb-1.2.1&auto=format&fit=crop&w=500&q=60".to_string(),
            case_study_url: "/case-studies/enterprise-dashboard".to_string(),
        },
        Project {
            id: "project-2".to_string(),
            title: "HealthTrack Mobile App".to_string(),
            category: "Mobile Application".to_string(),
            description: "A healthcare tracking application with advanced analytics capabilities.".to_string(),
            image_url: "https://images.unsplash.com/photo-1526498460520-4c246339dccb?ixlib=rb-1.2.1&auto=format&fit=crop&w=500&q=60".to_string(),
            case_study_url: "/case-studies/healthtrack-app".to_string(),
        },
        Project {
            id: "project-3".to_string(),
            title: "Smart Inventory System".to_string(),
            category: "Enterprise Solution".to_string(),
            description: "An automated inventory management system for retail chains.".to_string(),
            image_url: "https://images.unsplash.com/photo-1504868584819-f8e8b4b6d7e3?ixlib=rb-1.2.1&auto=format&fit=crop&w=500&q=60".to_string(),
            case_study_url: "/case-studies/smart-inventory".to_string(),
        },
        Project {
            id: "project-4".to_string(),
            title: "E-Commerce Platform".to_string(),
            category: "Web Application".to_string(),
            description: "A scalable e-commerce solution with integrated payment processing.".to_string(),
            image_url: "https://images.unsplash.com/photo-1556742049-0cfed4f6a45d?ixlib=rb-1.2.1&auto=format&fit=crop&w=500&q=60".to_string(),
            case_study_url: "/case-studies/ecommerce-platform".to_string(),
        },
    ]
}