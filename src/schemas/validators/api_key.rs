use std::fmt::{Display, Formatter};
use serde::{Deserialize, Deserializer, Serialize};
use regex::Regex;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Clone, ToSchema)]
#[schema(example = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef", pattern = r"^[0-9a-f]{64}$")]
pub struct ApiKey(String);

impl ApiKey {
    const REGEX: &'static str = r"^[0-9a-f]{64}$";

    fn validate(&self) -> Result<(), String> {
        let re = Regex::new(Self::REGEX).unwrap();
        re.is_match(&self.0).then(|| ()).ok_or(format!("Invalid api_key format: {}", self.0))
    }
}

impl<'de> Deserialize<'de> for ApiKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let auditory_id = ApiKey(s);
        auditory_id.validate().map_err(serde::de::Error::custom)?;
        Ok(auditory_id)
    }
}

impl Display for ApiKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
