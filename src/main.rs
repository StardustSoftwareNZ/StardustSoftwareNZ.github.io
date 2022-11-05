/// Main - main.rs 
/// ==============
/// The entry point for the applicaiton, it runs the application and sets up the router.

use yew_router::prelude::*;
use yew::prelude::*;

mod routes;
mod pages;

use crate::routes::{Route, switch};

/// The App component is the root component of the application.
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

// The entry point of the web application. 
fn main() {
    // Initialize the logger when the application is started. 
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}