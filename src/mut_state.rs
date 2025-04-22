use crate::schemas::graph::Graph;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct AppStateMutable {
    pub data_entry: Mutex<HashMap<String, Graph>>,
}

impl Default for AppStateMutable {
    fn default() -> Self {
        Self {
            data_entry: Mutex::new(Default::default())
        }
    }
}

