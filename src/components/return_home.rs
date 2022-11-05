/// Return Home - return_home.rs
/// ============================
/// This component is a button used to return to the home page.

use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;

#[function_component(ReturnHome)]
pub fn return_home() -> Html {
    // The history hook pushes routes to the browser history.
    let history = use_history().unwrap();

    // This handler navigates to the home page when the button is clicked.
    let onclick = Callback::once(move |_| history.push(Route::Home));

    html! {
        <button {onclick}>{ "🏠"}</button>
    }
}
