mod config;
use crate::mainnet::Client;
pub use config::CliConfig;
use config::{BeaconMethod, Namespace::Beacon};
use std::fmt;

use self::config::StateIdArg;

pub async fn run_cli(client: &Client, args: &CliConfig) -> Box<dyn fmt::Debug> {
    match &args.namespace {
        Beacon(BeaconMethod::Genesis) => Box::new(client.get_genesis_details().await),
        Beacon(BeaconMethod::Root(arg)) => {
            Box::new(client.get_state_root(arg.state_id.clone()).await)
        }
        Beacon(BeaconMethod::FinalityCheckpoints(arg)) => {
            Box::new(client.get_finality_checkpoints(arg.state_id.clone()).await)
        }
        Beacon(BeaconMethod::Validators(arg)) => Box::new(
            client
                .get_validators(
                    arg.state_id.to_owned(),
                    &[arg.validator_id.to_owned()],
                    &[arg.status],
                )
                .await,
        ),
    }
}
