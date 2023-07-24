use beacon_api_client::{mainnet::Client, run_cli, CliConfig, Error};
use clap::Parser;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), beacon_api_client::Error> {
    let args = CliConfig::parse();
    let url = Url::parse(&args.endpoint).unwrap();
    let client = Client::new(url);
    dbg!(run_cli(&client, &args).await)?;
    Ok(())
}
