use crate::pages;
/// Routes - mod.rs
/// ===============
/// We seperate the routing logic from the rest of the application.
/// This file is like __init__.py in Python. It is the entry point for the module.
/// It makes the each route module available to the rest of the application.
use yew::prelude::*;
use yew_router::prelude::*;

/// The route enum is used to define the routes for the application.
#[derive(Clone, Routable, PartialEq, Eq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/services/ai-solutions")]
    AiService,
    #[at("/services/web-development-solutions")]
    WebDevService,
    #[at("/services/cloud-solutions")]
    CloudService,
    #[at("/services/custom-solutions")]
    CustomService,
    #[at("/services/mobile-solutions")]
    MobileService,
    #[at("/services/ai-readiness-assessment")]
    AiReadinessAssessment,
    #[at("/services/cloud-readiness-assessment")]
    CloudReadinessAssessment,
    #[at("/services/free-needs-assessment")]
    FreeNeedsAssessment,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[at("/secure")]
    Secure,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// The switch component controls the routing of the application.
#[allow(clippy::let_unit_value)] // See https://github.com/rust-lang/rust-clippy/issues/9048
pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {<pages::home::Home />},
        Route::About => html! { <pages::about::About /> },
        Route::AiService => html! { <pages::ai_service::AiService /> },
        Route::WebDevService => html! { <pages::web_dev_service::WebDevService /> },
        Route::CloudService => html! { <pages::cloud_service::CloudService /> },
        Route::CustomService => html! { <pages::custom_service::CustomService /> },
        Route::MobileService => html! { <pages::mobile_service::MobileService /> },
        Route::AiReadinessAssessment => html! { <pages::ai_assessment::AiReadinessAssessment /> },
        Route::CloudReadinessAssessment => html! { <pages::cloud_assessment::CloudReadinessAssessment /> },
        Route::FreeNeedsAssessment => html! { <pages::custom_assessment::FreeNeedsAssessment /> },
        Route::Contact => html! { <pages::contact::Contact />},
        Route::Secure => html! { <pages::secure::Secure />},
        Route::Projects => html! { <pages::projects::Projects />},
        Route::NotFound => html! { <pages::error::Error />},
    }
}
