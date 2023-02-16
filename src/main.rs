use beacon_api_client::{run_cli, CliConfig, Client};
use clap::Parser;
use url::Url;

#[tokio::main]
async fn main() {
    let args = CliConfig::parse();
    let url: Url = Url::parse(&args.endpoint).unwrap();
    let client = Client::new(url);

    run_cli(client, args).await;
}
