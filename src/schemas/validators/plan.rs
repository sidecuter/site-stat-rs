use std::fmt::{Display, Formatter};
use serde::{Deserialize, Deserializer, Serialize};
use regex::Regex;
use utoipa::ToSchema;
use utoipauto::utoipa_ignore;

#[utoipa_ignore]
#[derive(Debug, Serialize, Clone, ToSchema)]
#[schema(example = "A-0", pattern = r"^([ABVN]D?-\d)$")]
pub struct PlanId(
    String
);

impl PlanId {
    const REGEX: &'static str = r"^([ABVN]D?-\d)$";

    fn validate(&self) -> Result<(), String> {
        let re = Regex::new(Self::REGEX).unwrap();
        re.is_match(&self.0).then(|| ()).ok_or(format!("Invalid plan_id format: {}", self.0))
    }

    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl<'de> Deserialize<'de> for PlanId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let plan_id = PlanId(s);
        plan_id.validate().map_err(serde::de::Error::custom)?;
        Ok(plan_id)
    }
}

impl Display for PlanId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for PlanId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for PlanId {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}
