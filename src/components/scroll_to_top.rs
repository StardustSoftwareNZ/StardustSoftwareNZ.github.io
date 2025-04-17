use yew::prelude::*;
use web_sys::{window, ScrollToOptions};

#[function_component(ScrollToTop)]
pub fn scroll_to_top() -> Html {
    // The closure is the first argument, dependencies are the second
    use_effect_with_deps(
        |_| {
            if let Some(window) = window() {
                let options = ScrollToOptions::new();
                let _ = options.set_top(0.0);
                let _ = window.scroll_with_scroll_to_options(&options);
            }
            
            // Return cleanup function (empty in this case)
            || ()
        },
        (), // Dependencies (empty tuple means run once on mount)
    );
    
    html! {}
}