use beacon_api_client::{Config, run_cli};
use clap::Parser;

#[tokio::main]
async fn main() {
    let config = Config::parse();
    run_cli(&config);
    //run_cli(&config).await;
    println!("{:?}\n{:?}\n{:?}\n{:?}", config.endpoint, config.namespace, config.method, config.args.unwrap_or("no args provided".to_string()))
}
