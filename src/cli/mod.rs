mod config;
use crate::{mainnet::Client};
pub use config::CliConfig;
use config::{BeaconMethod, Namespace::Beacon};
use std::fmt;

pub async fn run_cli(client: &Client, args: &CliConfig) -> Box<dyn fmt::Debug> {
    match &args.namespace {
        Beacon(BeaconMethod::Genesis) => Box::new(client.get_genesis_details().await),
        Beacon(BeaconMethod::Root(arg)) => {
            Box::new(client.get_state_root(arg.state_id.clone()).await)
        }
    }
}
