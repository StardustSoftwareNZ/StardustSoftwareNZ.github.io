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
            
            <style>
                {r#"
                * {
                    margin: 0;
                    padding: 0;
                    box-sizing: border-box;
                }
                
                .nav-container {
                    position: fixed;
                    top: 0;
                    left: 0;
                    width: 100%;
                    background-color: rgba(255, 255, 255, 0.98);
                    box-shadow: 0 2px 15px rgba(0, 0, 0, 0.08);
                    transition: all 0.3s ease;
                    z-index: 1000;
                    padding: 0.75rem 1.5rem;
                    backdrop-filter: blur(10px);
                }
                
                .nav-scrolled {
                    padding: 0.5rem 1.5rem;
                    background-color: rgba(255, 255, 255, 0.98);
                    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.12);
                }
                
                .nav-content {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    max-width: 1400px;
                    margin: 0 auto;
                }
                
                .nav-brand {
                    display: flex;
                    align-items: center;
                    gap: 1rem;
                }
                
                .nav-logo {
                    height: 2.8rem;
                    width: auto;
                    transition: all 0.3s ease;
                }
                
                .nav-scrolled .nav-logo {
                    height: 2.2rem;
                }
                
                .nav-company-name {
                    font-size: 1.6rem;
                    font-weight: 700;
                    background: linear-gradient(135deg, #3498db, #2c3e50);
                    -webkit-background-clip: text;
                    background-clip: text;
                    -webkit-text-fill-color: transparent;
                    text-decoration: none;
                    transition: all 0.3s ease;
                    letter-spacing: -0.5px;
                }
                
                .nav-scrolled .nav-company-name {
                    font-size: 1.4rem;
                }
                
                .nav-links-desktop {
                    display: flex;
                    align-items: center;
                    gap: 1.5rem;
                }
                
                .nav-link, .nav-link-active {
                    display: flex;
                    align-items: center;
                    gap: 0.5rem;
                    font-size: 1.05rem;
                    font-weight: 500;
                    color: #4a5568;
                    text-decoration: none;
                    padding: 0.6rem 1rem;
                    border-radius: 8px;
                    transition: all 0.25s ease;
                    position: relative;
                }
                
                .nav-link::after {
                    content: '';
                    position: absolute;
                    bottom: 0;
                    left: 50%;
                    width: 0;
                    height: 2px;
                    background: linear-gradient(90deg, #3498db, #2c3e50);
                    transition: all 0.3s ease;
                    transform: translateX(-50%);
                }
                
                .nav-link:hover {
                    color: #2d3748;
                    background-color: rgba(237, 242, 247, 0.7);
                }
                
                .nav-link:hover::after {
                    width: 70%;
                }
                
                .nav-link-active {
                    color: #2c3e50;
                    background-color: rgba(235, 248, 255, 0.7);
                    font-weight: 600;
                }
                
                .nav-link-active::after {
                    content: '';
                    position: absolute;
                    bottom: 0;
                    left: 50%;
                    width: 70%;
                    height: 2px;
                    background: linear-gradient(90deg, #3498db, #2c3e50);
                    transform: translateX(-50%);
                }
                
                .nav-icon {
                    font-size: 1.25rem;
                }
                
                .nav-contact-btn {
                    padding: 0.625rem 1.5rem;
                    font-size: 0.95rem;
                    font-weight: 600;
                    color: white;
                    background: linear-gradient(135deg, #3498db, #2c3e50);
                    border: none;
                    border-radius: 8px;
                    cursor: pointer;
                    transition: all 0.25s ease;
                    box-shadow: 0 4px 6px rgba(44, 62, 80, 0.15);
                }
                
                .nav-contact-btn:hover {
                    transform: translateY(-2px);
                    box-shadow: 0 6px 12px rgba(44, 62, 80, 0.2);
                }
                
                .nav-contact-btn:active {
                    transform: translateY(0);
                    box-shadow: 0 4px 6px rgba(44, 62, 80, 0.1);
                }
                
                .nav-mobile-toggle {
                    display: none;
                    font-size: 1.5rem;
                    background: transparent;
                    border: none;
                    color: #4a5568;
                    cursor: pointer;
                }
                
                .nav-mobile-menu {
                    display: none;
                    position: fixed;
                    top: 72px;
                    left: 0;
                    width: 100%;
                    background-color: white;
                    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.1);
                    transform: translateY(-100%);
                    transition: transform 0.3s ease;
                    z-index: 999;
                    padding: 1.5rem;
                    flex-direction: column;
                    gap: 1rem;
                    border-bottom-left-radius: 16px;
                    border-bottom-right-radius: 16px;
                }
                
                .nav-mobile-menu-open {
                    transform: translateY(0);
                }
                
                .nav-mobile-link {
                    display: flex;
                    align-items: center;
                    gap: 0.75rem;
                    font-size: 1.2rem;
                    font-weight: 500;
                    color: #4a5568;
                    text-decoration: none;
                    padding: 0.9rem 1rem;
                    border-radius: 8px;
                    transition: all 0.25s ease;
                }
                
                .nav-mobile-link:hover {
                    background-color: #f7fafc;
                    color: #2c3e50;
                }
                
                .nav-mobile-contact-btn {
                    margin-top: 0.75rem;
                    padding: 1rem 1.5rem;
                    font-size: 1.1rem;
                    font-weight: 600;
                    color: white;
                    background: linear-gradient(135deg, #3498db, #2c3e50);
                    border: none;
                    border-radius: 8px;
                    cursor: pointer;
                    transition: all 0.3s ease;
                    text-align: center;
                    box-shadow: 0 4px 6px rgba(44, 62, 80, 0.15);
                }
                
                .nav-mobile-contact-btn:hover {
                    background: linear-gradient(135deg, #2980b9, #1e2a37);
                }
                
                @media (max-width: 768px) {
                    .nav-links-desktop {
                        display: none;
                    }
                    
                    .nav-mobile-toggle {
                        display: block;
                    }
                    
                    .nav-mobile-menu {
                        display: flex;
                    }
                    
                    .nav-container {
                        padding: 0.75rem 1rem;
                    }
                    
                    .nav-company-name {
                        font-size: 1.4rem;
                    }
                }
                "#}
            </style>
        </nav>
    }
}