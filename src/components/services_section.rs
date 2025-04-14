use yew::prelude::*;
use crate::api::services::get_services;
use crate::components::section_header::SectionHeader;
use crate::components::service_card::ServiceCard;

#[derive(Properties, PartialEq)]
pub struct ServicesSectionProps {
    #[prop_or_default]
    pub id: Option<String>,
}

/// Services section component to display all service offerings
#[function_component(ServicesSection)]
pub fn services_section(props: &ServicesSectionProps) -> Html {
    let services = get_services();
    
    let id_attr = props.id.clone().unwrap_or("services".to_string());
    
    html! {
        <section id={id_attr} class="services-section">
            <div class="container">
                <SectionHeader 
                    title="Our Services" 
                    subtitle={Some("Comprehensive software solutions tailored to your needs".to_string())} 
                />
                
                <div class="services-grid">
                    {
                        services.into_iter().map(|service| {
                            html! {
                                <ServiceCard key={service.id.clone()} service={service.clone()} />
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </section>
    }
}