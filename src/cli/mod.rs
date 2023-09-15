mod config;
use crate::{mainnet::Client, Error};
pub use config::CliConfig;
use config::{BeaconMethod, Namespace::Beacon};
use std::fmt;

pub async fn run_cli(client: &Client, args: &CliConfig) -> Result<Box<dyn fmt::Debug>, Error> {
    match &args.namespace {
        Beacon(BeaconMethod::Genesis) => {
            let result = client.get_genesis_details().await?;
            Ok(Box::new(result))
        }
        Beacon(BeaconMethod::Root(arg)) => {
            let result = client.get_state_root(&arg.state_id).await?;
            Ok(Box::new(result))
        }
        Beacon(BeaconMethod::FinalityCheckpoints(arg)) => {
            let result = client.get_finality_checkpoints(arg.state_id.clone()).await?;
            Ok(Box::new(result))
        }
        Beacon(BeaconMethod::Validators(arg)) => {
            let result = client
                .get_validators(
                    arg.state_id.to_owned(),
                    &[arg.validator_id.to_owned()],
                    &[arg.status],
                )
                .await?;
            Ok(Box::new(result))
        }
    }
}
