pub mod api_client;
use serde_json;
use tokio::main;

#[tokio::main]
async fn main() {

    let ip_addr = "localhost";
    let port_id = "8002";
    let endpoint = "eth/v1/node/version";
    let state_id = "";

    let url = format!("http://{}:{}/{}/{}", ip_addr, port_id, endpoint, state_id);
    let client = api_client::BeaconClient::new(reqwest::Client::new());
    
    let version = client.get_node_version(ip_addr, port_id, endpoint, state_id).await;

    println!("{:?}", version.unwrap().to_string());

}
