use std::fmt::{Display, Formatter};
use serde::{Deserialize, Deserializer, Serialize};
use utoipa::ToSchema;
use utoipauto::utoipa_ignore;

#[utoipa_ignore]
#[derive(Debug, Serialize, Clone, ToSchema)]
#[schema(example = 50)]
pub struct Size(u64);

impl Size {

    fn validate(&self) -> Result<(), String> {
        if self.0 > 100 {
            Err(format!("Size value greater than 100. Current value: {}", self.0))
        } else {
            Ok(())
        }
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

impl<'de> Deserialize<'de> for Size {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: u64 = Deserialize::deserialize(deserializer)?;
        let size = Size(s);
        size.validate().map_err(serde::de::Error::custom)?;
        Ok(size)
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Size {
    fn default() -> Self {
        Self(50)
    }
}
