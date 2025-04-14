use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::{MouseEvent, Window};
use gloo::events::EventListener;

/// FancyNavigation - fancy_navigation.rs
/// =====================================
/// A fancy, professional navigation component for a software engineering business.
/// Features responsive design, gradient effects, active route highlighting, and smooth animations.

#[derive(Properties, PartialEq)]
pub struct NavigationProps {
    #[prop_or_default]
    pub logo_url: Option<String>,
    #[prop_or("Stardust Software NZ".to_string())]
    pub company_name: String,
}

#[function_component(Navigation)]
pub fn fancy_navigation(props: &NavigationProps) -> Html {
    // We can access navigator from context in recent versions of yew-router
    // let navigator = use_navigator().unwrap();
    let route = use_route::<Route>().unwrap_or(Route::Home);
    let mobile_menu_open = use_state(|| false);
    
    let toggle_mobile_menu = {
        let mobile_menu_open = mobile_menu_open.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            mobile_menu_open.set(!*mobile_menu_open);
        })
    };
    
    let close_mobile_menu = {
        let mobile_menu_open = mobile_menu_open.clone();
        Callback::from(move |_| {
            mobile_menu_open.set(false);
        })
    };
    
    let is_active = |r: Route| -> String {
        if route == r {
            "nav-link-active".to_string()
        } else {
            "nav-link".to_string()
        }
    };

    // Handle scroll behavior
    let scroll_state = use_state(|| false);

    {
        let scroll_state = scroll_state.clone();
        use_effect_with_deps(
            move |_| {
                let window = web_sys::window().expect("no global `window` exists");
                
                // Clone the scroll_state to avoid borrowing issues
                let state_clone = scroll_state.clone();
                
                let scroll_listener = EventListener::new(&window, "scroll", move |_| {
                    // Use the cloned state
                    if let Ok(y) = web_sys::window().expect("no window").scroll_y() {
                        if y > 50.0 {
                            state_clone.set(true);
                        } else {
                            state_clone.set(false);
                        }
                    }
                });
                
                // Return disposer function
                move || {
                    drop(scroll_listener);
                }
            },
            (),
        );
    }

    let nav_class = if *scroll_state {
        "nav-container nav-scrolled"
    } else {
        "nav-container"
    };

    html! {
        <nav class={nav_class}>
            <div class="nav-content">
                <div class="nav-brand">
                    if let Some(logo) = &props.logo_url {
                        <Link<Route> to={Route::Home} classes={classes!("nav-logo-link")}>
                            <img src={logo.clone()} alt="Company Logo" class="nav-logo" />
                        </Link<Route>>
                    }
                    <Link<Route> to={Route::Home} classes={classes!("nav-company-name")}>
                        { &props.company_name }
                    </Link<Route>>
                </div>

                // Desktop Navigation
                <div class="nav-links-desktop">
                    <Link<Route> to={Route::Home} classes={classes!(is_active(Route::Home))}>
                        <span class="nav-icon">{ "🏠" }</span>
                        <span class="nav-text">{ "Home" }</span>
                    </Link<Route>>
                    <Link<Route> to={Route::Projects} classes={classes!(is_active(Route::Projects))}>
                        <span class="nav-icon">{ "🛠️" }</span>
                        <span class="nav-text">{ "Projects" }</span>
                    </Link<Route>>
                    <Link<Route> to={Route::About} classes={classes!(is_active(Route::About))}>
                        <span class="nav-icon">{ "ℹ️" }</span>
                        <span class="nav-text">{ "About" }</span>
                    </Link<Route>>
                    <Link<Route> to={Route::Contact} classes={classes!(is_active(Route::Contact))}>
                    <span class="nav-icon">{ "📧" }</span>
                    <span class="nav-text">{ "Contact" }</span>
                </Link<Route>>
                </div>

                // Mobile Menu Button
                <button class="nav-mobile-toggle" onclick={toggle_mobile_menu.clone()}>
                    if *mobile_menu_open {
                        { "✕" }
                    } else {
                        { "☰" }
                    }
                </button>
            </div>

            // Mobile Navigation Menu
            <div class={if *mobile_menu_open { "nav-mobile-menu nav-mobile-menu-open" } else { "nav-mobile-menu" }}>
                <div onclick={close_mobile_menu.clone()}>
                    <Link<Route> to={Route::Home} classes={classes!(format!("nav-mobile-link {}", is_active(Route::Home)))}>
                        <span class="nav-icon">{ "🏠" }</span>
                        <span class="nav-text">{ "Home" }</span>
                    </Link<Route>>
                </div>
                <div onclick={close_mobile_menu.clone()}>
                    <Link<Route> to={Route::Projects} classes={classes!(format!("nav-mobile-link {}", is_active(Route::Projects)))}>
                        <span class="nav-icon">{ "🛠️" }</span>
                        <span class="nav-text">{ "Projects" }</span>
                    </Link<Route>>
                </div>
                <div onclick={close_mobile_menu.clone()}>
                    <Link<Route> to={Route::About} classes={classes!(format!("nav-mobile-link {}", is_active(Route::About)))}>
                        <span class="nav-icon">{ "ℹ️" }</span>
                        <span class="nav-text">{ "About" }</span>
                    </Link<Route>>
                </div>
                <div onclick={close_mobile_menu.clone()}>
                    <Link<Route> to={Route::Secure} classes={classes!(format!("nav-mobile-link {}", is_active(Route::Secure)))}>
                        <span class="nav-icon">{ "💻" }</span>
                        <span class="nav-text">{ "Secure" }</span>
                    </Link<Route>>
                </div>
                <div onclick={close_mobile_menu.clone()}>
                    <Link<Route> to={Route::Contact} classes={classes!(format!("nav-mobile-link {}", is_active(Route::Contact)))}>
                        <span class="nav-icon">{ "📧" }</span>
                        <span class="nav-text">{ "Contact" }</span>
                    </Link<Route>>
                </div>
            </div>
        </nav>
    }
}