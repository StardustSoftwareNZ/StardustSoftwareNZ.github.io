use std::env;

fn main() {
    // Load .env file
    dotenv::dotenv().ok();
    
    // Forward environment variables to the build process
    if let Ok(supabase_url) = env::var("SUPABASE_URL") {
        println!("cargo:rustc-env=SUPABASE_URL={}", supabase_url);
    }
    
    if let Ok(supabase_key) = env::var("SUPABASE_KEY") {
        println!("cargo:rustc-env=SUPABASE_KEY={}", supabase_key);
    }
}