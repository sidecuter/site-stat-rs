pub mod api;
pub mod api_docs;
pub mod app_state;
pub mod cors;
pub mod entity;
pub mod errors;
mod macros;
pub mod middleware;
pub mod mut_state;
pub mod schemas;
pub mod task;
pub mod traits;

#[cfg(test)]
mod tests;
