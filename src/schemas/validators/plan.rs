use regex::Regex;
use std::sync::LazyLock;

pub static PLAN_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([ABVN]D?-\d)$").unwrap());
