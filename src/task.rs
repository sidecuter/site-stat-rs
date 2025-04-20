use std::time::Duration;
use actix_rt::time;
use actix_web::web;
use crate::mut_state::AppStateMutable;
use crate::schemas::data::get_graphs;

pub async fn start_data_refresh_task(
    state: web::Data<AppStateMutable>,
    interval: Duration
) {
    let mut interval = time::interval(interval);
    loop {
        interval.tick().await;
        if let Err(e) = refresh_data(&state).await {
            tracing::warn!("Refresh failed: {}", e);
        }
    }
}

async fn refresh_data(state: &web::Data<AppStateMutable>) -> Result<(), Box<dyn std::error::Error>> {
    let new_entry = get_graphs().await?;
    let mut entry = state.data_entry.lock()
        .map_err(|v| format!("{v}"))?;
    *entry = new_entry;
    Ok(())
}
