pub mod api_client;
use serde_json;


fn main() {
    // set basic vars and get api key from secret
    let port_id: String = "8002".to_string();
    let state_id: String = "".to_string();
    let ip_addr: String = "localhost".to_string();
    let endpoint: String = "eth/v1/node/version".to_string();

    let version = api_client::http_request(
        &ip_addr,
        &endpoint,
        &state_id,
        &port_id
    );

    println!(
        "version: {:?}", version.unwrap()
    );

}
