use beacon_api_client::{run_cli, Config};
use clap::Parser;

#[tokio::main]
async fn main() {
    let config = Config::parse();
    println!("\nReturn val: ");
    run_cli(&config).await;

    println!("\nCLI args:");
    println!(
        "endpoint: {:?}\nnamespace: {:?}\nmethod: {:?}\nargs: {:?}",
        config.endpoint,
        config.namespace,
        config.method,
        config.args.unwrap_or("no args provided".to_string())
    );
    println!("");
}
