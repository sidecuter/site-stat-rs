use crate::mut_state::AppStateMutable;
use crate::schemas::data::get_graphs;
use actix_rt::time;
use actix_web::web;
use std::time::Duration;

pub async fn start_data_refresh_task(state: web::Data<AppStateMutable>, interval: Duration) {
    let mut interval = time::interval(interval);
    loop {
        interval.tick().await;
        if let Err(e) = refresh_data(&state).await {
            tracing::warn!("Refresh failed: {}", e);
        }
    }
}

async fn refresh_data(
    state: &web::Data<AppStateMutable>,
) -> Result<(), Box<dyn std::error::Error>> {
    let new_entry = get_graphs().await?;
    let mut entry = state.data_entry.lock().map_err(|v| format!("{v}"))?;
    *entry = new_entry;
    Ok(())
}
