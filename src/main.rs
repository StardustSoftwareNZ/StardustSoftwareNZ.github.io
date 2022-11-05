/// Main - main.rs 
/// ==============
/// The entry point for the applicaiton, it runs the application and sets up the router.

use yew_router::prelude::*;
use yew::prelude::*;

// Import the pages as a module. Each page is in the pages directory.
// These are correspond to the routes in the Route enum.
mod pages;

/// The route enum is used to define the routes for the application.
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("about")]
    About, 
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// The switch component controls the routing of the application.
fn switch(routes: &Route) -> Html {
    match routes { 
        Route::Home => html! { <pages::home::Home /> },
        Route::About => html! { <pages::about::About /> },
        Route::Secure => html! { <pages::secure::Secure />},
        Route::NotFound => html! { <pages::error::Error />},
    }
}

/// The App component is the root component of the application.
/// This serves the browser router using a switch component.
#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} /> 
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}