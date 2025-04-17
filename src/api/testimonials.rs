use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use dotenv_codegen::dotenv;
use gloo_utils::format::JsValueSerdeExt;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Testimonial {
    pub id: String,
    pub text: String,
    pub author_name: String,
    pub author_title: String,
    pub author_company: String,
    pub author_image: String,
}

// Constants from .env file
const SUPABASE_URL: &str = dotenv!("SUPABASE_URL");
const SUPABASE_API_KEY: &str = dotenv!("SUPABASE_KEY");

// Function to get testimonials from Supabase
pub async fn fetch_testimonials() -> Result<Vec<Testimonial>, JsValue> {
    // Create request object
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    
    // Configure request with Supabase headers
    let url = format!("{}/rest/v1/testimonials?select=*", SUPABASE_URL);
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
        return Err(JsValue::from_str(&format!("Error fetching testimonials: {}", status)));
    }
    
    // Parse the JSON response
    let json = JsFuture::from(resp.json()?).await?;
    let testimonials: Vec<Testimonial> = json.into_serde().map_err(|e| {
        JsValue::from_str(&format!("Error deserializing testimonials: {}", e))
    })?;
    
    Ok(testimonials)
}

// Fallback to get hardcoded testimonials in case the API fails
pub fn get_hardcoded_testimonials() -> Vec<Testimonial> {
    vec![
        Testimonial {
            id: "testimonial-1".to_string(),
            text: "Stardust Software transformed our business with their custom CRM solution. Their team was professional, responsive, and delivered beyond our expectations.".to_string(),
            author_name: "Donald Trump".to_string(),
            author_title: "President".to_string(),
            author_company: "United States of America".to_string(),
            author_image: "https://upload.wikimedia.org/wikipedia/commons/thumb/8/83/TrumpPortrait.jpg/250px-TrumpPortrait.jpg".to_string(),
        },
        Testimonial {
            id: "testimonial-2".to_string(),
            text: "The mobile app developed by Stardust Software has revolutionized how we engage with our customers. The attention to detail and user experience is outstanding.".to_string(),
            author_name: "Elon Musk".to_string(),
            author_title: "CEO Founder".to_string(),
            author_company: "SpaceX, Tesla, X".to_string(),
            author_image: "https://futureoflife.org/wp-content/uploads/2020/08/elon_musk_royal_society.jpg".to_string(),
        },
        Testimonial {
            id: "testimonial-3".to_string(),
            text: "Working with Stardust Software was a game-changer for our startup. Their expertise in cloud solutions helped us scale rapidly without compromising security.".to_string(),
            author_name: "Joe Biden".to_string(),
            author_title: "President".to_string(),
            author_company: "United States of America".to_string(),
            author_image: "https://upload.wikimedia.org/wikipedia/commons/thumb/6/68/Joe_Biden_presidential_portrait.jpg/1200px-Joe_Biden_presidential_portrait.jpg".to_string(),
        }
    ]
}

// Public function that components will call (backward compatibility)
pub fn get_testimonials() -> Vec<Testimonial> {
    // Return the hardcoded testimonials initially
    // The actual component using this should implement the async fetch
    get_hardcoded_testimonials()
}