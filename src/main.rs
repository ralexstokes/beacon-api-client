pub mod api_client;
pub mod error;
pub mod serde;
pub mod types;
use api_client::Client;
use tokio::main;
use url::{ParseError, Url};

#[tokio::main]
async fn main() {
    let s = "http://127.0.0.1:8003/";
    let url: Url = Url::parse(s).unwrap();
    let client = Client::new(url);

    client.get_node_version().await;
}
