/// About - about.rs
/// ===============
/// This is the about page. It gives a brief description of what the company does. 

use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div>
            <h1>{ "About" }</h1>
        </div>
    }
}