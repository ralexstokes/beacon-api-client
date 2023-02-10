use beacon_api_client::{run_cli, CliConfig, Client};
use clap::Parser;
use url::Url;

#[tokio::main]
async fn main() {
    // read in args from CLI
    let args = CliConfig::parse();
    // instantiate client and pass to run_cli
    let url: Url = Url::parse(&args.endpoint).unwrap();
    let client = Client::new(url);
    run_cli(client, args).await;
}
