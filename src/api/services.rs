use serde::{Deserialize, Serialize};
use crate::routes::Route;

#[derive(Clone, PartialEq, Debug)]
pub struct Service {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: ServiceIcon,
    pub route: Route,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ServiceIcon {
    pub viewbox: String,
    pub path: String,
}

pub fn get_services() -> Vec<Service> {
    vec![
        Service {
            id: "service-0".to_string(),
            title: "Artificial Intelligence Solutions".to_string(),
            description: "Implementing cutting-edge AI technologies to transform your business operations, enhance decision-making, and create intelligent applications that deliver competitive advantages.".to_string(),
            icon: ServiceIcon {
                viewbox: "0 0 24 24".to_string(),
                path: "M21 11.18V9.72c0-.47-.16-.92-.46-1.28L16.6 3.72c-.38-.46-.94-.72-1.54-.72H8.94c-.6 0-1.15.26-1.54.72L3.46 8.44c-.3.36-.46.81-.46 1.28v1.45c0 .8.65 1.45 1.45 1.45H4v6c0 .55.45 1 1 1h14c.55 0 1-.45 1-1v-6h.55c.8 0 1.45-.65 1.45-1.45zM7.58 5h8.84l2.26 2.73H5.32L7.58 5zM15 15v1H9v-1H7v-2h10v2h-2zm-4 3v-1h2v1h-2z M7.58 10.2l.4.4-.4.4-3.6-3.6L7.58 4h8.84l3.6 3.4-3.6 3.6-.4-.4.4-.4.6-.6H6.98l.6.6z".to_string(),
            },
            route: Route::AiService,
        },
        Service {
            id: "service-1".to_string(),
            title: "Web Development".to_string(),
            description: "Creating responsive, fast, and intuitive web applications using modern frameworks and best practices.".to_string(),
            icon: ServiceIcon {
                viewbox: "0 0 24 24".to_string(),
                path: "M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm-5 14H4v-4h11v4zm0-5H4V9h11v4zm5 5h-4V9h4v9z".to_string(),
            },
            route: Route::WebDevService,
        },
        Service {
            id: "service-2".to_string(),
            title: "Mobile Applications".to_string(),
            description: "Building native and cross-platform mobile apps that deliver exceptional user experiences.".to_string(),
            icon: ServiceIcon {
                viewbox: "0 0 24 24".to_string(),
                path: "M17 1.01L7 1c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM17 19H7V5h10v14z".to_string(),
            },
            route: Route::MobileService,
        },
        Service {
            id: "service-3".to_string(),
            title: "Cloud Solutions".to_string(),
            description: "Designing scalable, secure, and reliable cloud infrastructures for your business needs.".to_string(),
            icon: ServiceIcon {
                viewbox: "0 0 24 24".to_string(),
                path: "M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96zM19 18H6c-2.21 0-4-1.79-4-4 0-2.05 1.53-3.76 3.56-3.97l1.07-.11.5-.95C8.08 7.14 9.94 6 12 6c2.62 0 4.88 1.86 5.39 4.43l.3 1.5 1.53.11c1.56.1 2.78 1.41 2.78 2.96 0 1.65-1.35 3-3 3z".to_string(),
            },
            route: Route::CloudService,
        },
        Service {
            id: "service-4".to_string(),
            title: "Custom Solutions".to_string(),
            description: "Developing tailored software solutions that address your unique business challenges.".to_string(),
            icon: ServiceIcon {
                viewbox: "0 0 24 24".to_string(),
                path: "M9 21c0 .55.45 1 1 1h4c.55 0 1-.45 1-1v-1H9v1zm3-19C8.14 2 5 5.14 5 9c0 2.38 1.19 4.47 3 5.74V17c0 .55.45 1 1 1h6c.55 0 1-.45 1-1v-2.26c1.81-1.27 3-3.36 3-5.74 0-3.86-3.14-7-7-7zm2.85 11.1l-.85.6V16h-4v-2.3l-.85-.6C7.8 12.16 7 10.63 7 9c0-2.76 2.24-5 5-5s5 2.24 5 5c0 1.63-.8 3.16-2.15 4.1z".to_string(),
            },
            route: Route::CustomService,
        },
    ]
}