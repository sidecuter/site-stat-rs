use crate::mut_state::AppStateMutable;
use crate::schemas::data::get_graphs;
use actix_rt::time;
use actix_web::web;
use std::time::Duration;

pub async fn start_data_refresh_task(state: web::Data<AppStateMutable>, interval: Duration) {
    let mut interval = time::interval(interval);
    loop {
        interval.tick().await;
        let _ = refresh_data(&state)
            .await
            .map_err(|e| {
                tracing::error!("Refresh failed: {}", e);
            })
            .map(|()| tracing::info!("Data refreshed"));
    }
}

#[allow(clippy::significant_drop_tightening)]
async fn refresh_data(
    state: &web::Data<AppStateMutable>,
) -> Result<(), Box<dyn std::error::Error>> {
    let new_entry = get_graphs().await?;
    let mut entry = state.data_entry.lock().map_err(|v| format!("{v}"))?;
    *entry = new_entry;
    Ok(())
}
