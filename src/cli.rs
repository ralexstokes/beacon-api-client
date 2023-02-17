use crate::{
    api_client::Client,
    cli_types::{BeaconMethod, CliConfig, Namespace::Beacon},
};

pub async fn run_cli(client: Client, args: CliConfig) {
    match args.command {
        Beacon(BeaconMethod::Genesis(genesis)) => genesis.execute(&client).await,
        Beacon(BeaconMethod::Root(ref state_id)) => state_id.execute(&client).await,
        Beacon(BeaconMethod::FinalityCheckpoints(ref state_id)) => state_id.execute(&client).await,
        Beacon(BeaconMethod::Validator(ref validator_args)) => {
            validator_args.execute(&client).await
        }
        Beacon(BeaconMethod::Validators(ref validators_args)) => {
            validators_args.execute(&client).await
        }
        Beacon(BeaconMethod::ValidatorBalances(ref validator_balances_args)) => {
            validator_balances_args.execute(&client).await
        }
        // Committees(CommitteesArg),
        // SyncCommittees(SyncCommitteesArg),
        _ => println!("coming later"),
    }
}
