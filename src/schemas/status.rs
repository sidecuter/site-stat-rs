use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct Status {
    pub status: String
}

impl Default for Status {
    fn default() -> Self {
        return Self{status: "OK".to_string()}
    }
}
