use yew::prelude::*;
use crate::api::services::Service;

#[derive(Properties, PartialEq)]
pub struct ServiceCardProps {
    pub service: Service,
}

/// Service card component to display individual service offerings
#[function_component(ServiceCard)]
pub fn service_card(props: &ServiceCardProps) -> Html {
    let service = &props.service;
    
    html! {
        <div class="service-card">
            <div class="service-icon">
                <svg viewBox={service.icon.viewbox.clone()} width="64" height="64">
                    <path d={service.icon.path.clone()} fill="#4facfe"/>
                </svg>
            </div>
            <h3 class="service-title">{ &service.title }</h3>
            <p class="service-description">{ &service.description }</p>
            <a href={service.link.clone()} class="service-link">{"Learn More"}</a>
        </div>
    }
}