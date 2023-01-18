use beacon_api_client::{cli, CliArgs};
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    cli(args).await;
}
