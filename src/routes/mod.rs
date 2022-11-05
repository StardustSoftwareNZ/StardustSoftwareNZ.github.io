use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages;

/// The route enum is used to define the routes for the application.
#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About, 
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// The switch component controls the routing of the application.
pub fn switch(routes: &Route) -> Html {
    match routes { 
        Route::Home => html! { <pages::home::Home /> },
        Route::About => html! { <pages::about::About /> },
        Route::Secure => html! { <pages::secure::Secure />},
        Route::NotFound => html! { <pages::error::Error />},      
    }
}
