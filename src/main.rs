mod services;

use crate::services::go_plus::GoPlusService;
use crate::services::http_client::HttpClient;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let http_client = HttpClient::new();
    let go_plus_service = GoPlusService::new(http_client);
    let result = go_plus_service
        .get_token_security("0x17837004ea685690b32dbead02a274ec4333a26a", 1)
        .await?;
    println!("GoPlus Response: {:?}", result);
    Ok(())
}
