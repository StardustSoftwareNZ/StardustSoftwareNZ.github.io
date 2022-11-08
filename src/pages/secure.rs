use crate::components::navigation::Navigation;
use crate::components::return_home::ReturnHome;
/// Secure - secure.rs
/// =================
/// This is the secure page. It is displayed when the user tries to access a page that requires authentication.
use yew::prelude::*;

#[function_component(Secure)]
pub fn secure() -> Html {
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <Navigation />
            <p>{ "This is a secure page. ( ͡° ͜ʖ ͡°)" }</p>
            <ReturnHome/>
        </div>
    }
}
