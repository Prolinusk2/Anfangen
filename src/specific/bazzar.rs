use std::error::Error;
#[derive(Debug)]
pub struct BazzarClient{
    pub full_json : serde_json::Value
}

impl BazzarClient {
    pub async fn new() -> Result<Self, Box<dyn Error + Send + Sync>>{
        let full_json = crate::specific::online_manager::get_bazzar().await?;
        Ok(Self { full_json })
    }

    pub fn all_items(&self) -> Vec<String>{
        self.full_json["products"].as_object().unwrap().keys().cloned().collect()
    }

    pub fn item_quick_status(&self, item_name: &str) -> serde_json::Value{
        self.full_json["products"][item_name]["quick_status"].clone()
    }
}