// src/config.rs
use std::env;

// These will be set at compile time
pub const SUPABASE_URL: &str = env!("SUPABASE_URL", "SUPABASE_URL not set");
pub const SUPABASE_KEY: &str = env!("SUPABASE_KEY", "SUPABASE_KEY not set");