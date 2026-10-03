use std::error::Error;
use std::fs;

fn get_api_key() -> Result<String, Box<dyn Error + Send + Sync>> {
    // Kein Fallback auf "1": ein ungültiger Key liefert HTTP 200 mit
    // `success: false`, was wie "Spieler hat keine Items" aussieht.
    match fs::read_to_string("API_KEY") {
        Ok(key) if !key.trim().is_empty() => Ok(key.trim().to_string()),
        _ => Err("API_KEY-Datei fehlt oder ist leer".into()),
    }
}

/// Hypixel meldet Fehler mit HTTP 200 und `{"success": false, "cause": "..."}`.
/// Ohne diese Prüfung verschluckt ein `#[serde(default)]` den Fehler und ein
/// Spieler mit API-Fehler landet als "0 Items, 0 Fehler" in der DB.
pub fn pruefe_antwort(wert: &serde_json::Value) -> Result<(), Box<dyn Error + Send + Sync>> {
    if wert["success"].as_bool() == Some(false) {
        let cause = wert["cause"].as_str().unwrap_or("unbekannter API-Fehler");
        return Err(format!("Hypixel-API: {cause}").into());
    }
    Ok(())
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

    pruefe_antwort(&json)?;

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

    pruefe_antwort(&antwort)?;

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

    pruefe_antwort(&antwort)?;

    Ok(antwort)
}
