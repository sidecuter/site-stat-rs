use std::sync::Mutex;
use crate::schemas::data::DataEntry;

pub struct AppStateMutable {
    pub data_entry: Mutex<DataEntry>
}
