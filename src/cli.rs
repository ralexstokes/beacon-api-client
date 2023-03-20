use crate::{
    api_client::Client,
    cli_types::{BeaconMethod, CliConfig, Namespace::Beacon},
    HeaderArg,
};

pub async fn run_cli(client: Client, args: CliConfig) {
    match args.command {
        Beacon(BeaconMethod::Genesis(genesis)) => genesis.execute(&client).await,
        Beacon(BeaconMethod::Root(ref root_arg)) => root_arg.execute(&client).await,
        Beacon(BeaconMethod::FinalityCheckpoints(ref finality_checkpoints_arg)) => {
            finality_checkpoints_arg.execute(&client).await
        }
        Beacon(BeaconMethod::Validator(ref validator_args)) => {
            validator_args.execute(&client).await
        }
        Beacon(BeaconMethod::Validators(ref validators_args)) => {
            validators_args.execute(&client).await
        }
        Beacon(BeaconMethod::ValidatorBalances(ref validator_balances_args)) => {
            validator_balances_args.execute(&client).await
        }
        Beacon(BeaconMethod::Committees(ref committees_args)) => {
            committees_args.execute(&client).await
        }
        Beacon(BeaconMethod::SyncCommittees(ref sync_committees_args)) => {
            sync_committees_args.execute(&client).await
        }
        Beacon(BeaconMethod::HeaderAtHead(header_arg)) => header_arg.execute(&client).await,
        Beacon(BeaconMethod::HeaderForSlot(header_arg)) => {
            println!(
                "Method not yet functional due to unresolved error parsing headers in api_client!"
            )
        }
        Beacon(BeaconMethod::HeaderForParentRoot(header_arg)) => {
            println!(
                "Method not yet functional due to unresolved error parsing headers in api_client!"
            )
        }
        Beacon(BeaconMethod::HeaderForBlockId(header_arg)) => {
            println!(
                "Method not yet functional due to unresolved error parsing headers in api_client!"
            )
        }
        Beacon(BeaconMethod::Block(block_arg)) => block_arg.execute(&client).await,
        Beacon(BeaconMethod::PostBlock(post_block_arg)) => {
            println!("Method not yet functional - need to work out POSTs and marshalling SignedBeaconBlock type")
        }
        Beacon(BeaconMethod::BlockRoot(block_root_arg)) => block_root_arg.execute(&client).await,
        Beacon(BeaconMethod::BlockAttestations(block_attestations_arg)) => {
            block_attestations_arg.execute(&client).await
        }
        Beacon(BeaconMethod::PoolAttestations(pool_attestations_arg)) => {
            pool_attestations_arg.execute(&client).await
            // note that to skip one or other arg, pass <None> as positional cli arg
        }
        Beacon(BeaconMethod::AttesterSlashing(attester_slashing_arg)) => {
            attester_slashing_arg.execute(&client).await
        }
        Beacon(BeaconMethod::ProposerSlashing(proposer_slashing_arg)) => {
            proposer_slashing_arg.execute(&client).await
        }
        _ => println!("method not yet implemented"),
    }
}
