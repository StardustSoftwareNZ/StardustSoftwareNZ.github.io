/// Home - home.rs
/// ==============
/// This is the home page. It is the first page that the user sees when they visit the website.

use yew::prelude::*;
use yew_router::prelude::*;

/// Import route enum from main.rs
use crate::routes::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| { history.push(Route::About) });

    html! {
        <div>
            <h1>{ "Home" }</h1>
            <button {onclick}>{ "About" }</button>
        </div>
    }
}