/// Home - home.rs
/// ==============
/// This is the home page. It is the first page that the user sees when they visit the website.

use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{ "Home" }</h1>
        </div>
    }
}