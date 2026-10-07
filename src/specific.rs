use std::error::Error;
use crate::specific::bazzar::BazzarClient;
use crate::specific::online_manager::get_profile;
use crate::specific::player::Player;

mod online_manager;
mod bazzar;
mod player;

pub async fn ausführen() -> Result<(), Box<dyn Error + Send + Sync>> {
    let brazzers = BazzarClient::new().await?;
    let player = Player::new("Prolinusk2").await?;

    let profile = get_profile(&player.uuid, &player.aktive_profile_id).await?;


    println!("{:#}", profile);

    Ok(())
}