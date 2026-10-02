use serde_json::json;

pub mod online_manager;

pub async fn get_all_bazzar_items() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let bazaar_json = online_manager::get_bazzar().await?;
    let mut item_list = vec![];

    match bazaar_json["products"].as_object() {
        Some(items) => {
            for (item, value) in items {
                item_list.push(item.clone());
            }
        }
        None => {return Ok(vec![])}
    }
    println!("{:?}", item_list);
    Ok(item_list)
}

pub async fn get_quik_status_specific_item(item: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let bazzar_json = online_manager::get_bazzar().await?;

    Ok(bazzar_json["products"][item]["quick_status"].clone())
}