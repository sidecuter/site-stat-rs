use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter};
use utoipa::ToSchema;
use utoipauto::utoipa_ignore;

#[utoipa_ignore]
#[derive(Debug, Serialize, Clone, ToSchema)]
#[schema(example = "a-100", pattern = r"^(!?[abvn]d?(-\w+)*)$")]
pub struct AuditoryId(String);

impl AuditoryId {
    const REGEX: &'static str = r"^(!?[abvn]d?(-\w+)*)$";

    fn validate(&self) -> Result<(), String> {
        let re = Regex::new(Self::REGEX).unwrap();
        re.is_match(&self.0)
            .then(|| ())
            .ok_or(format!("Invalid auditory_id format: {}", self.0))
    }

    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl<'de> Deserialize<'de> for AuditoryId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let auditory_id = AuditoryId(s);
        auditory_id.validate().map_err(serde::de::Error::custom)?;
        Ok(auditory_id)
    }
}

impl Display for AuditoryId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for AuditoryId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for AuditoryId {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}
