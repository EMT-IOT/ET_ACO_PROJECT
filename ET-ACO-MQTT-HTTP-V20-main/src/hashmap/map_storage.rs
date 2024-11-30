use std::collections::HashMap;
use std::sync::{Arc,Mutex};

#[derive(Clone)]
pub struct HaspMapService {
    pub map:Arc<Mutex<HashMap<String,String>>>
}

impl HaspMapService {
    pub fn init() -> Self {
        Self {
            map:Arc::new(Mutex::new(HashMap::new()))
        }
    }
}