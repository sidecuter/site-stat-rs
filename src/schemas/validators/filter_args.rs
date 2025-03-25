use regex::Regex;
use std::sync::LazyLock;

pub static API_KEY_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[0-9a-f]{64}$").unwrap()
});

const DEFAULT_SIZE: u64 = 50;
const DEFAULT_PAGE: u64 = 1;

pub fn size_default() -> u64 {
    DEFAULT_SIZE
}

pub fn page_default() -> u64 {
    DEFAULT_PAGE
}
