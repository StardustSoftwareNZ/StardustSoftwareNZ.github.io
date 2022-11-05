use yew::prelude::*;
use yew_router::prelude::*;

// Import route enum from main.
use crate::routes::{Route};

#[function_component(Secure)]
pub fn secure() -> Html {
    // The history hook pushes routes to the browser history.
    let history = use_history().unwrap();

    // This hanlder navigates to the home page when the button is clicked.
    let onclick = Callback::once(move |_| history.push(Route::Home));

    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home"}</button>
        </div>
    }
}