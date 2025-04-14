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
            author_name: "Donald Trump".to_string(),
            author_title: "President".to_string(),
            author_company: "United States of America".to_string(),
            author_image: "https://upload.wikimedia.org/wikipedia/commons/thumb/8/83/TrumpPortrait.jpg/250px-TrumpPortrait.jpg".to_string(),
        },
        Testimonial {
            id: "testimonial-2".to_string(),
            text: "The mobile app developed by Stardust Software has revolutionized how we engage with our customers. The attention to detail and user experience is outstanding.".to_string(),
            author_name: "Elon Musk".to_string(),
            author_title: "CEO Founder".to_string(),
            author_company: "SpaceX, Tesla, X".to_string(),
            author_image: "https://futureoflife.org/wp-content/uploads/2020/08/elon_musk_royal_society.jpg".to_string(),
        },
        Testimonial {
            id: "testimonial-3".to_string(),
            text: "Working with Stardust Software was a game-changer for our startup. Their expertise in cloud solutions helped us scale rapidly without compromising security.".to_string(),
            author_name: "Joe Biden".to_string(),
            author_title: "President".to_string(),
            author_company: "United States of America".to_string(),
            author_image: "https://upload.wikimedia.org/wikipedia/commons/thumb/6/68/Joe_Biden_presidential_portrait.jpg/1200px-Joe_Biden_presidential_portrait.jpg".to_string(),
        }
    ]
}