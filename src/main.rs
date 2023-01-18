use beacon_api_client::{parseCli, CliArgs, Client, StateId};
use clap::Parser;
use url::Url;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    parseCli(args).await;
}
