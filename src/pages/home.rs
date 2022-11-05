/// Home - home.rs
/// ==============
/// This is the home page. It is the first page that the user sees when they visit the website.

use yew::prelude::*;
use crate::components::navigation::Navigation;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{ "Home" } </h1>
            <Navigation />
            <p> { "Welcome to Stardust Software NZ."} </p>
            <p> { "Work in progress..."} </p>
        </div>
    }
}
