use yew::prelude::*;
use crate::components::section_header::SectionHeader;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ProcessStep {
    pub id: String,
    pub number: String,
    pub title: String,
    pub description: String,
}

#[derive(Properties, PartialEq)]
pub struct ProcessSectionProps {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub steps: Option<Vec<ProcessStep>>,
    #[prop_or("Our Process".to_string())]
    pub title: String,
    #[prop_or("How we turn your ideas into reality".to_string())]
    pub subtitle: String,
}

/// Process section component to display workflow steps
#[function_component(ProcessSection)]
pub fn process_section(props: &ProcessSectionProps) -> Html {
    let id_attr = props.id.clone().unwrap_or("process".to_string());
    
    // Default process steps if none provided
    let steps = match &props.steps {
        Some(steps) => steps.clone(),
        None => vec![
            ProcessStep {
                id: "step-1".to_string(),
                number: "01".to_string(),
                title: "Discovery".to_string(),
                description: "We start by understanding your business, goals, and challenges through in-depth consultations.".to_string(),
            },
            ProcessStep {
                id: "step-2".to_string(),
                number: "02".to_string(),
                title: "Planning".to_string(),
                description: "Our team creates a comprehensive roadmap with clear milestones and deliverables.".to_string(),
            },
            ProcessStep {
                id: "step-3".to_string(),
                number: "03".to_string(),
                title: "Development".to_string(),
                description: "We build your solution using agile methodologies, ensuring quality at every step.".to_string(),
            },
            ProcessStep {
                id: "step-4".to_string(),
                number: "04".to_string(),
                title: "Testing".to_string(),
                description: "Rigorous testing ensures your software performs flawlessly across all scenarios.".to_string(),
            },
            ProcessStep {
                id: "step-5".to_string(),
                number: "05".to_string(),
                title: "Deployment".to_string(),
                description: "We launch your solution and provide comprehensive training and documentation.".to_string(),
            },
            ProcessStep {
                id: "step-6".to_string(),
                number: "06".to_string(),
                title: "Support".to_string(),
                description: "Our relationship continues with ongoing maintenance and support services.".to_string(),
            },
        ],
    };
    
    html! {
        <section id={id_attr} class="process-section">
            <div class="container">
                <SectionHeader 
                    title={props.title.clone()}
                    subtitle={Some(props.subtitle.clone())}
                />
                <div class="process-timeline">
                    {
                        steps.into_iter().map(|step| {
                            html! {
                                <div key={step.id.clone()} class="process-step">
                                    <div class="step-number">{ step.number }</div>
                                    <div class="step-content">
                                        <h3 class="step-title">{ step.title }</h3>
                                        <p class="step-description">{ step.description }</p>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </section>
    }
}