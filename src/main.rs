use ecb_exchange::ecb_url;
use reqwest::Client;
use std::error::Error;

use ecb_exchange::parsing::parse;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let xml_content = client.get(ecb_url::TODAY).send().await?.text().await?;
    let parsed = parse(&xml_content).unwrap();
    println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
    Ok(())
}
