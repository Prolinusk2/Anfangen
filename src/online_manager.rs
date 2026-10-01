use std::fs;

fn get_api_key() -> String {
    fs::read_to_string("src/API_KEY").unwrap_or("1".to_string())
}

pub async fn get_recent_games(uuid: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let antwort = client
        .get("https://api.hypixel.net/v2/recentgames")
        .query(&[("uuid", uuid)])
        .header("API-Key", get_api_key())
        .send()
        .await?
        .text()
        .await?;

    Ok(antwort)
}

pub async fn get_player_status(uuid: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let antwort = client
        .get("https://api.hypixel.net/v2/status")
        .query(&[("uuid", uuid)])
        .header("API-Key", get_api_key())
        .send()
        .await?
        .text()
        .await?;

    Ok(antwort)
}

pub async fn get_uuid(username: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let url = format!("https://api.mojang.com/users/profiles/minecraft/{username}");

    let json: serde_json::Value = client.get(url).send().await?.json().await?;

    Ok(json["id"].as_str().unwrap_or_default().to_string())
}

pub async fn get_bazzar() -> Result<serde_json::Value, reqwest::Error> {
    let client = reqwest::Client::new();

    let antwort = client
        .get("https://api.hypixel.net/v2/skyblock/bazaar")
        .header("API-Key", get_api_key())
        .send()
        .await?
        .json()
        .await?;

    Ok(antwort)
}