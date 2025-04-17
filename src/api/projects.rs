use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Request, RequestInit, RequestMode, Response};
use dotenv_codegen::dotenv;
use gloo_utils::format::JsValueSerdeExt; // Add this for into_serde
use yew::prelude::*;

// Define the Project struct the same as before
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub category: String,
    pub description: String,
    pub image_url: String,
    pub case_study_url: String,
}

// Constants from .env file
const SUPABASE_URL: &str = dotenv!("SUPABASE_URL");
const SUPABASE_API_KEY: &str = dotenv!("SUPABASE_KEY");

// Function to get projects from Supabase
pub async fn fetch_projects() -> Result<Vec<Project>, JsValue> {
    // Create request object
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    
    // Configure request with Supabase headers
    let url = format!("{}/rest/v1/projects?select=*", SUPABASE_URL);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    
    // Add required headers for Supabase
    let headers = request.headers();
    headers.set("apikey", SUPABASE_API_KEY)?;
    headers.set("Authorization", &format!("Bearer {}", SUPABASE_API_KEY))?;
    headers.set("Content-Type", "application/json")?;
    headers.set("Prefer", "return=representation")?;
    
    // Send the request
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    
    // Check for successful response
    if !resp.ok() {
        let status = resp.status();
        return Err(JsValue::from_str(&format!("Error fetching projects: {}", status)));
    }
    
    // Parse the JSON response
    let json = JsFuture::from(resp.json()?).await?;
    let projects: Vec<Project> = json.into_serde().map_err(|e| {
        JsValue::from_str(&format!("Error deserializing projects: {}", e))
    })?;
    
    Ok(projects)
}

// Fallback to get hardcoded projects in case the API fails
pub fn get_hardcoded_projects() -> Vec<Project> {
    vec![
        Project {
            id: "project-1".to_string(),
            title: "Portfolio Website".to_string(),
            category: "Web Application".to_string(),
            description: "A portfolio website for an academic, made with Deno Fresh, Postgresql and Supabase.".to_string(),
            image_url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse4.mm.bing.net%2Fth%3Fid%3DOIP._46ZR0zkVmoRwaMuSdDFXQHaEo%26pid%3DApi&f=1&ipt=aff04108aa1e9a287c52a3584357be0c54b95167122f03357e1de7886be0c0bd".to_string(),
            case_study_url: "https://woodrock.deno.dev/".to_string(),
        },
        Project {
            id: "project-2".to_string(),
            title: "Wellington Bus Timetable".to_string(),
            category: "Web Application".to_string(),
            description: "A bus timetable for the Wellington Region, made with Deno Fresh and Metlink API.".to_string(),
            image_url: "https://media.gettyimages.com/id/1501353205/photo/double-decker-metlink-bus-te-whanganui-a-tara-wellington-spring-day.jpg?s=2048x2048&w=gi&k=20&c=aIXWGu9Hu8TAfdf4bLmj22whHJVtGV12WujZ80WkOtk=".to_string(),
            case_study_url: "https://bus-timetable.deno.dev/".to_string(),
        },
        Project {
            id: "project-3".to_string(),
            title: "Renewed Roots".to_string(),
            category: "Web Application".to_string(),
            description: "An online shop for selling recycled furniture, made with Deno Fresh".to_string(),
            image_url: "https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fwww.architectureartdesigns.com%2Fwp-content%2Fuploads%2F2016%2F03%2F18-Remarkable-Furniture-Designs-Made-From-Recycled-Pallet-Wood-13.jpg&f=1&nofb=1&ipt=452ac8479c837abdd46a689d9a2f630bfca9caf44cfd8844be6acec775925037".to_string(),
            case_study_url: "https://flogging-furniture.deno.dev/".to_string(),
        },
        Project {
            id: "project-4".to_string(),
            title: "Ionic Scholar".to_string(),
            category: "Mobile Application".to_string(),
            description: "A citations tracker for academics made with Ionic React Framework.".to_string(),
            image_url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Funissa.edu.bn%2Fjournal%2Fpublic%2Fsite%2Fimages%2Froot%2Fgoogle-scholar-d727840d5d57573b2c18a8dcb280ee96.png&f=1&nofb=1&ipt=2a69517dd0930eb0e4445b2568b7b3b2b026fa4c4c5a2f3efc5f35c6c2807370".to_string(),
            case_study_url: "https://github.com/woodRock/ionic-scholar".to_string(),
        },
        Project {
            id: "project-5".to_string(),
            title: "Wordle Solver Transformer".to_string(),
            category: "Artificial Intelligence".to_string(),
            description: "A worlde solver built with python, pytorch, and deno fresh".to_string(),
            image_url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fgamingonphone.com%2Fwp-content%2Fuploads%2F2022%2F01%2FScreenshot-2022-01-11-at-7.49.02-PM-scaled.jpg&f=1&nofb=1&ipt=17fcf0024fd2a58185820e3fe994be8e4509de7755084e783bf8883ab3c14be1".to_string(),
            case_study_url: "https://github.com/woodRock/wordle".to_string(),
        },
        Project {
            id: "project-6".to_string(),
            title: "Autograd".to_string(),
            category: "Artificial Intelligence".to_string(),
            description: "Deep learning library written in c++ and cuda for a basic autograd.".to_string(),
            image_url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fsslprod.oss-cn-shanghai.aliyuncs.com%2Fstable%2Fslides%2F04_Back_propagation_and_PyTorch_autograd%2F04_Back_propagation_and_PyTorch_autograd_1440-05.jpg&f=1&nofb=1&ipt=10660cae9b14ef211218747f70091d219345ee03a81a1007bc6e66183d948de3".to_string(),
            case_study_url: "https://github.com/woodRock/autograd".to_string(),
        }
    ]
}

// Public function that components will call
pub fn get_projects() -> Vec<Project> {
    // Return the hardcoded projects initially
    // The actual component using this should implement the async fetch
    get_hardcoded_projects()
}

// Example component usage (this would be in your projects component file)
#[function_component(ProjectsComponent)]
pub fn projects_component() -> Html {
    let projects = use_state(|| Vec::<Project>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    
    // Effect to fetch projects on component mount
    {
        let projects = projects.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    match fetch_projects().await {
                        Ok(fetched_projects) => {
                            projects.set(fetched_projects);
                            loading.set(false);
                        },
                        Err(err) => {
                            // Log the error
                            web_sys::console::error_1(&err);
                            // Fallback to hardcoded projects
                            projects.set(get_hardcoded_projects());
                            error.set(Some(format!("Error: {:?}", err)));
                            loading.set(false);
                        }
                    }
                });
                
                || ()
            },
            (), // Empty dependencies - only run once on mount
        );
    }
    
    // Render the projects
    html! {
        <div class="projects-container">
            if *loading {
                <p>{"Loading projects..."}</p>
            } else {
                if let Some(err_msg) = &*error {
                    <div class="error-message">
                        <p>{format!("Error loading from API (using fallback data): {}", err_msg)}</p>
                    </div>
                }
                <div class="projects-grid">
                    {
                        projects.iter().map(|project| {
                            html! {
                                <div key={project.id.clone()} class="project-card">
                                    <h3>{&project.title}</h3>
                                    <p>{&project.category}</p>
                                    <p>{&project.description}</p>
                                    // Add image, links, etc.
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            }
        </div>
    }
}