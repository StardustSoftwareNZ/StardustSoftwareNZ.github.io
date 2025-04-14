use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Testimonial {
    pub id: String,
    pub text: String,
    pub author_name: String,
    pub author_title: String,
    pub author_company: String,
    pub author_image: String,
}

pub fn get_testimonials() -> Vec<Testimonial> {
    vec![
        Testimonial {
            id: "testimonial-1".to_string(),
            text: "Stardust Software transformed our business with their custom CRM solution. Their team was professional, responsive, and delivered beyond our expectations.".to_string(),
            author_name: "Sarah Johnson".to_string(),
            author_title: "CEO".to_string(),
            author_company: "TechInnovate".to_string(),
            author_image: "https://randomuser.me/api/portraits/women/45.jpg".to_string(),
        },
        Testimonial {
            id: "testimonial-2".to_string(),
            text: "The mobile app developed by Stardust Software has revolutionized how we engage with our customers. The attention to detail and user experience is outstanding.".to_string(),
            author_name: "Michael Chang".to_string(),
            author_title: "Marketing Director".to_string(),
            author_company: "GlobalReach".to_string(),
            author_image: "https://randomuser.me/api/portraits/men/32.jpg".to_string(),
        },
        Testimonial {
            id: "testimonial-3".to_string(),
            text: "Working with Stardust Software was a game-changer for our startup. Their expertise in cloud solutions helped us scale rapidly without compromising security.".to_string(),
            author_name: "Emily Roberts".to_string(),
            author_title: "CTO".to_string(),
            author_company: "NexGen Solutions".to_string(),
            author_image: "https://randomuser.me/api/portraits/women/22.jpg".to_string(),
        },
        Testimonial {
            id: "testimonial-4".to_string(),
            text: "The team at Stardust Software exceeded our expectations. They took the time to understand our complex requirements and delivered a solution that perfectly matches our needs.".to_string(),
            author_name: "David Wilson".to_string(),
            author_title: "Operations Manager".to_string(),
            author_company: "Logistics Pro".to_string(),
            author_image: "https://randomuser.me/api/portraits/men/67.jpg".to_string(),
        },
    ]
}