use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SectionHeaderProps {
    pub title: String,
    #[prop_or_default]
    pub subtitle: Option<String>,
    #[prop_or_default]
    pub class: Classes,
}

/// Reusable section header component with title, divider, and optional subtitle
#[function_component(SectionHeader)]
pub fn section_header(props: &SectionHeaderProps) -> Html {
    html! {
        <div class={classes!("section-header", props.class.clone())}>
            <h2 class="section-title">{ &props.title }</h2>
            <div class="section-divider"></div>
            if let Some(subtitle) = &props.subtitle {
                <p class="section-subtitle">{ subtitle }</p>
            }
        </div>
    }
}