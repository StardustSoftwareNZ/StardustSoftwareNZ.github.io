use yew::prelude::*;
use crate::components::section_header::SectionHeader;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Statistic {
    pub id: String,
    pub number: String,
    pub label: String,
}

#[derive(Properties, PartialEq)]
pub struct AboutSectionProps {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub paragraphs: Option<Vec<String>>,
    #[prop_or_default]
    pub statistics: Option<Vec<Statistic>>,
    #[prop_or_default]
    pub image_url: Option<String>,
    #[prop_or("About Stardust Software NZ".to_string())]
    pub title: String,
    #[prop_or_default]
    pub subtitle: Option<String>,
}

/// About section component for company information
#[function_component(AboutSection)]
pub fn about_section(props: &AboutSectionProps) -> Html {
    let id_attr = props.id.clone().unwrap_or("about".to_string());
    
    // Default paragraphs if none provided
    let paragraphs = match &props.paragraphs {
        Some(paragraphs) => paragraphs.clone(),
        None => vec![
            "At Stardust Software NZ, we blend creativity with technical expertise to deliver cutting-edge software and artificial intelligence solutions. Based in New Zealand, we work with clients globally to transform their digital presence.".to_string(),
            "Our one man team is committed to crafting software that not only meets your business requirements but exceeds your expectations.".to_string(),
        ],
    };
    
    // Default statistics if none provided
    let statistics = match &props.statistics {
        Some(statistics) => statistics.clone(),
        None => vec![
            Statistic {
                id: "stat-1".to_string(),
                number: "5+".to_string(),
                label: "Projects Completed".to_string(),
            },
            Statistic {
                id: "stat-2".to_string(),
                number: "1+".to_string(),
                label: "Team Members".to_string(),
            },
            Statistic {
                id: "stat-3".to_string(),
                number: "99%".to_string(),
                label: "Client Satisfaction".to_string(),
            },
        ],
    };
    
    html! {
        <section id={id_attr} class="about-section">
            <div class="container">
                <SectionHeader 
                    title={props.title.clone()}
                    subtitle={props.subtitle.clone()}
                />
                <div class="about-grid">
                    <div class="about-content">
                        {
                            paragraphs.into_iter().map(|paragraph| {
                                html! {
                                    <p class="about-text">{ paragraph }</p>
                                }
                            }).collect::<Html>()
                        }
                        <div class="company-stats">
                            {
                                statistics.into_iter().map(|stat| {
                                    html! {
                                        <div key={stat.id.clone()} class="stat-item">
                                            <span class="stat-number">{ stat.number }</span>
                                            <span class="stat-label">{ stat.label }</span>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    </div>
                    <div class="about-image">
                        <div class="image-container">
                            {
                                if let Some(url) = &props.image_url {
                                    html! {
                                        <img src={url.clone()} alt="About our company" class="about-img" />
                                    }
                                } else {
                                    html! {
                                        <>
                                            <div class="floating-shape shape1"></div>
                                            <div class="floating-shape shape2"></div>
                                            <div class="floating-shape shape3"></div>
                                        </>
                                    }
                                }
                            }
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}