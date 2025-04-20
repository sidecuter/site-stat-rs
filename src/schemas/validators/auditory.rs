use regex::Regex;
use std::sync::LazyLock;

pub static AUD_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(!?[abvn]d?(-\w+)*)$").unwrap());
