use crate::schemas::graph::Graph;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct AppStateMutable {
    pub data_entry: Mutex<HashMap<String, Graph>>,
}
