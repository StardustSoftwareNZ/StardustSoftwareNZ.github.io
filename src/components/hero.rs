use yew::prelude::*;
use web_sys::MouseEvent;

#[derive(Properties, PartialEq)]
pub struct HeroProps {
    pub title: String,
    pub subtitle: String,
    #[prop_or_default]
    pub primary_button_text: Option<String>,
    #[prop_or_default]
    pub primary_button_onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub secondary_button_text: Option<String>,
    #[prop_or_default]
    pub secondary_button_onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub scroll_down_id: Option<String>,
    #[prop_or_default]
    pub scroll_down_text: Option<String>,
    #[prop_or_default]
    pub background_image: Option<String>,
}

/// Hero banner component with background, title, subtitle, and action buttons
#[function_component(Hero)]
pub fn hero(props: &HeroProps) -> Html {
    let scroll_to_section = |id: String| {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(element) = document.get_element_by_id(&id) {
                        element.scroll_into_view_with_bool(true);
                    }
                }
            }
        })
    };

    let background_style = match &props.background_image {
        Some(image_url) => format!("background-image: url('{}');", image_url),
        None => String::new(),
    };

    html! {
        <section class="hero-section">
            <div class="hero-overlay" style={background_style}></div>
            <div class="hero-content">
                <h1 class="hero-title">{ &props.title }</h1>
                <h2 class="hero-subtitle">{ &props.subtitle }</h2>
                
                <div class="hero-buttons">
                    if let Some(text) = &props.primary_button_text {
                        <button class="btn btn-primary" 
                                onclick={props.primary_button_onclick.clone().unwrap_or_default()}>
                            { text }
                        </button>
                    }
                    
                    if let Some(text) = &props.secondary_button_text {
                        <button class="btn btn-secondary" 
                                onclick={props.secondary_button_onclick.clone().unwrap_or_default()}>
                            { text }
                        </button>
                    }
                </div>
            </div>
            
            if let Some(id) = &props.scroll_down_id {
                <div class="hero-scrolldown" onclick={scroll_to_section(id.clone())}>
                    <span>{ props.scroll_down_text.clone().unwrap_or("Discover More".to_string()) }</span>
                    <div class="chevron"></div>
                </div>
            }
        </section>
    }
}