mod specific;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let quik = specific::get_quik_status_specific_item("WHITE_GIFT").await.unwrap();
    println!("{:#}", quik);
    Ok(())
}
