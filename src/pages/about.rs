use crate::components::return_home::ReturnHome;
/// About - about.rs
/// ===============
/// This is the about page. It gives a brief description of what the company does.
use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div>
            <h1>{ "About" }</h1>
            <p>{ "Stardust Software NZ is a software development company based in New Zealand." }</p>
            <p>{ "We are currently working on a new website." }</p>
            <ReturnHome />
        </div>
    }
}
