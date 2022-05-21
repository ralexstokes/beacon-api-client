mod api_client;
mod error;
mod serde;
mod types;
use api_client::Client;
use url::Url;

#[tokio::main]
async fn main() {
    let s = "http://127.0.0.1:8003/";
    let url: Url = Url::parse(s).unwrap();
    let client = Client::new(url);

    let version = client.get_node_version("http://localhost:8003/eth/v1/node/version").await;

    println!("node version:\n{:?}", version);
}
