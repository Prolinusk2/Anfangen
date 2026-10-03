use std::error::Error;
use crate::specific::bazzar::BazzarClient;

mod online_manager;
mod bazzar;
mod player;

pub async  fn ausführen() -> Result<(), Box<dyn Error + Send + Sync>> {
    let brazzers = BazzarClient::new().await?;

    println!("{:#}", brazzers.item_quick_status("GREEN_GIFT"));

    Ok(())
}