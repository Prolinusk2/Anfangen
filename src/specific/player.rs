use std::error::Error;
use crate::specific::online_manager;

#[derive(Debug)]
pub struct Player{
    pub uuid : String,
    pub player_profiles : serde_json::Value,
    pub aktive_profile_id : String
}

impl Player{
    pub async fn new(player: &str) -> Result<Self, Box<dyn Error + Send + Sync>>{
        let uuid =  online_manager::get_uuid(player).await?;
        let player_profiles = online_manager::get_profiles(&uuid).await?;
        let aktive_profile_id = Self::get_aktiv_profile_id(&player_profiles);
        Ok(Self { uuid, player_profiles, aktive_profile_id })
    }

    fn get_aktiv_profile_id(player_profiles : &serde_json::Value) -> String{
        let profiles = player_profiles["profiles"].as_array().unwrap();

        for profil in profiles{
            if profil["selected"].as_bool() == Some(true){
                return profil["profile_id"].as_str().unwrap().to_string();
            }
        }

        String::new()
    }
    
    fn get_all_player_items(&self){
        
    }

}
