use std::error::Error;

mod specific;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _hallo = specific::ausführen().await.unwrap();
    Ok(())
}
