mod online_manager;
mod specific;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let nummer = online_manager::get_bazzar().await?;
    let text = &nummer["products"]["FACTION_RABBIT_WALKER"]["quick_status"];
    
    println!("{:#}", text);
    Ok(())
}
