use yew::prelude::*;

struct Model {
    value: i64,
}

#[function_component(App)]
fn app() -> Html {
    // To create a closure in rust, use to pipe operators `||`.
    // That tells rust the next thing is the contents of our closure.
    let state = use_state(|| Model {
        value: 0
    });

    // Create a function that will increment the value.
    let onclick = {
        // The original state variable is used by yew.
        // We create a new variable to shadow it.
        let state = state.clone(); 

        // Pass a closure with our logic permutating state into this function. 
        // Callback takes one parameter, we don't care about it, so we use `_`.
        // By default closures in rust borrow references. But in our case we want to take ownership of the state, so we use `move`.
        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1
            })
        })
        // No semicolon, since we want the last line of the closure to be returned (similar to ruby).
    };

    html! {
        <div>
            <h1> {"Stardust Software NZ"} </h1>
            <button {onclick}>{"+🍪"}</button>
            <p>{format!("{:?} 🍪", state.value)}</p>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}