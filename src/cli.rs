use crate::{
    api_client::Client,
    cli_types::{BeaconMethod, CliConfig, Namespace::Beacon},
    types::{PublicKeyOrIndex, StateId},
};

pub async fn run_cli(client: Client, args: CliConfig) {
    match args.command {
        Beacon(BeaconMethod::Genesis(genesis)) => genesis.execute(&client).await,
        Beacon(BeaconMethod::Root(ref state_id)) => state_id.execute(&client).await,
        Beacon(BeaconMethod::FinalityCheckpoints(ref state_id)) => state_id.execute(&client).await,
        // Validator(ValidatorArg),
        // Validators(ValidatorsArg),
        // ValidatorBalances(ValidatorBalancesArg),
        // Committees(CommitteesArg),
        // SyncCommittees(SyncCommitteesArg),
        _ => println!("coming later"),
    }
}
