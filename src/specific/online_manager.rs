use std::error::Error;
use std::fmt::format;
use std::fs;

fn get_api_key() -> Result<String, Box<dyn Error + Send + Sync>> {
    match fs::read_to_string("API_KEY") {
        Ok(key) if !key.trim().is_empty() => Ok(key.trim().to_string()),
        _ => Err("API_KEY-Datei fehlt oder ist leer".into()),
    }
}

pub async fn get_recent_games(uuid: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let client = reqwest::Client::new();

    let antwort = client
        .get("https://api.hypixel.net/v2/recentgames")
        .query(&[("uuid", uuid)])
        .header("API-Key", get_api_key()?)
        .send()
        .await?
        .text()
        .await?;

    Ok(antwort)
}

pub async fn get_player_status(uuid: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let client = reqwest::Client::new();

    let antwort = client
        .get("https://api.hypixel.net/v2/status")
        .query(&[("uuid", uuid)])
        .header("API-Key", get_api_key()?)
        .send()
        .await?
        .text()
        .await?;

    Ok(antwort)
}

pub async fn get_uuid(username: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let client = reqwest::Client::new();

    let url = format!("https://api.mojang.com/users/profiles/minecraft/{username}");

    let json: serde_json::Value = client.get(url).send().await?.json().await?;

    json["id"]
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| format!("Mojang lieferte keine UUID für {username}").into())
}

pub async fn get_profiles(uuid: &str) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let api_key = get_api_key()?;

    let url = format!("https://api.hypixel.net/v2/skyblock/profiles?uuid={uuid}&key={api_key}");
    let antwort: serde_json::Value = client.get(url).send().await?.json().await?;

    Ok(antwort)
}

pub async fn get_profile(_player_uuid: &str, aktive_profile_id: &str) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
    let client = reqwest::Client::new();

    let antwort: serde_json::Value = client
        .get("https://api.hypixel.net/v2/skyblock/profile")
        .query(&[("profile", aktive_profile_id)])
        .header("API-Key", get_api_key()?)
        .send()
        .await?
        .json()
        .await?;

    Ok(antwort)
}

pub async fn get_bazzar() -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
    let client = reqwest::Client::new();

    let antwort: serde_json::Value = client
        .get("https://api.hypixel.net/v2/skyblock/bazaar")
        .header("API-Key", get_api_key()?)
        .send()
        .await?
        .json()
        .await?;
    Ok(antwort)
}
