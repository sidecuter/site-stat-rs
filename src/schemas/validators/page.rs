use std::fmt::{Display, Formatter};
use serde::{Deserialize, Deserializer, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Clone, ToSchema)]
#[schema(example = 1)]
pub struct Page(u64);

impl Page {

    fn validate(&self) -> Result<(), String> {
        if self.0 < 1 {
            Err(format!("Page value lesser than 1. Current value: {}", self.0))
        } else {
            Ok(())
        }
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

impl<'de> Deserialize<'de> for Page {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: u64 = Deserialize::deserialize(deserializer)?;
        let page = Page(s);
        page.validate().map_err(serde::de::Error::custom)?;
        Ok(page)
    }
}

impl Display for Page {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Page {
    fn default() -> Self {
        Self(1)
    }
}
