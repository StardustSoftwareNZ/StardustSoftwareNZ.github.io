use crate::components::navigation::Navigation;
use crate::components::footer::Footer;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::MouseEvent;

/// NotFound - not_found.rs
/// ====================
/// 404 page for Stardust Software NZ
#[function_component(Error)]
pub fn not_found() -> Html {
    let history = use_history().unwrap();
    
    // Space-themed random facts
    let space_facts = vec![
        "The Sun makes up 99.86% of the mass in our solar system.",
        "There are more stars in the universe than grains of sand on all the beaches on Earth.",
        "One million Earths could fit inside the Sun.",
        "The Milky Way galaxy contains over 100 billion stars.",
        "A day on Venus is longer than a year on Venus.",
        "Neutron stars are so dense that a teaspoon would weigh about 10 million tons.",
        "There is a planet made of diamonds, called 55 Cancri e.",
        "The footprints left on the Moon will last for 100 million years.",
    ];
    
    // Get a random fact
    let random_index = js_sys::Math::floor(js_sys::Math::random() * space_facts.len() as f64) as usize;
    let random_fact = space_facts[random_index];
    
    // Go home button handler
    let onclick = Callback::once(move |_| history.push(Route::Home));

    html! {
        <div class="not-found-page">
            // Navigation
            <Navigation 
                logo_url={"https://avatars.githubusercontent.com/u/80566578?s=200&v=4"} 
                company_name={"Stardust Software NZ"} 
            />
            
            <div class="space-background">
                <div class="star-field">
                    <div class="layer"></div>
                    <div class="layer"></div>
                    <div class="layer"></div>
                </div>
                
                <div class="not-found-content">
                    <div class="planet-container">
                        <div class="planet">
                            <div class="crater crater-1"></div>
                            <div class="crater crater-2"></div>
                            <div class="crater crater-3"></div>
                            <div class="crater crater-4"></div>
                            <div class="crater crater-5"></div>
                            <div class="satellite-orbit">
                                <div class="satellite"></div>
                            </div>
                        </div>
                    </div>
                    
                    <div class="text-container">
                        <h1 class="error-code">{"404"}</h1>
                        <h2 class="error-title">{"Houston, We Have a Problem"}</h2>
                        <p class="error-message">{"The page you're looking for has drifted off into space."}</p>
                        
                        <div class="space-fact">
                            <h3>{"Space Fact"}</h3>
                            <p>{random_fact}</p>
                        </div>
                        
                        <div class="action-buttons">
                            <button onclick={&onclick} class="btn btn-primary">{"Return to Home"}</button>
                            <button onclick={onclick} class="btn btn-secondary">{"Go Back"}</button>
                        </div>
                    </div>
                </div>
            </div>
            
            // Footer
            <Footer 
                company_name="Stardust Software NZ"
                logo_url={"https://avatars.githubusercontent.com/u/80566578?s=200&v=4".to_string()}
                tagline="Creating stellar software solutions for the modern world."
                copyright_year={2025}
            />
        </div>
    }
}