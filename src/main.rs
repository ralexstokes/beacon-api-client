use beacon_api_client::{cli, CliArgs, Client, StateId};
use clap::Parser;
use url::Url;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    cli(args).await;
}
