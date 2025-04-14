use crate::routes::Route;
/// Service Card - service_card.rs
/// ==================
/// This component displays an individual service offering.
use yew::prelude::*;
use yew_router::prelude::*;
use crate::api::services::Service;
    
#[derive(Properties, PartialEq)]
pub struct ServiceCardProps {
    pub service: Service,
}

/// Service card component to display individual service offerings
#[function_component(ServiceCard)]
pub fn service_card(props: &ServiceCardProps) -> Html {
    let service = &props.service;

    // The history hook pushes routes to the browser history.
    let history = use_history().unwrap();

    // This handler navigates to the home page when the button is clicked.
    let onclick = Callback::once(move |_| history.push(Route::AiService));
    
    html! {
        <div class="service-card">
            <div class="service-icon">
                <svg viewBox={service.icon.viewbox.clone()} width="64" height="64">
                    <path d={service.icon.path.clone()} fill="#4facfe"/>
                </svg>
            </div>
            <h3 class="service-title">{ &service.title }</h3>
            <p class="service-description">{ &service.description }</p>
            <a onclick={onclick} class="service-link">{"Learn More"}</a>
        </div>
    }
}