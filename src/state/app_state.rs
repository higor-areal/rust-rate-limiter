use std::collections::HashMap;
use crate::limiter::token_bucket::Bucket;

pub struct AppState{
    pub buckets: HashMap<String, Bucket>,
}

impl AppState{
    pub fn new()-> Self{
        AppState { buckets: HashMap::new() }
    }
}