use crate::routes::Route;
use crate::api::services::Service;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ServiceCardProps {
    pub service: Service,
}

#[function_component(ServiceCard)]
pub fn service_card(props: &ServiceCardProps) -> Html {
    let service = &props.service;
    let history = use_history().unwrap();
    let service_route = service.route.clone();
    let onclick = Callback::once(move |_| history.push(service_route));

    html! {
        <div class="service-card-wrapper"> // Added wrapper div
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
        </div>
    }
}