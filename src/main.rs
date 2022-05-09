pub mod api_client;
use serde_json;


fn main() {


    let version = api_client::get_node_version().unwrap().to_string();

    println!(
        "version: {}", version
    );

}
