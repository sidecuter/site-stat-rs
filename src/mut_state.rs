use std::collections::HashMap;
use std::sync::Mutex;
use crate::schemas::graph::Graph;

pub struct AppStateMutable {
    pub data_entry: Mutex<HashMap<String, Graph>>
}
