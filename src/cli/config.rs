use crate::{types::StateId, PublicKeyOrIndex, ValidatorStatus};
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CliConfig {
    #[arg(long)]
    pub endpoint: String,
    #[command(subcommand)]
    pub namespace: Namespace,
}

#[derive(Debug, Subcommand)]
pub enum Namespace {
    #[clap(subcommand)]
    Beacon(BeaconMethod),
}

#[derive(Debug, Subcommand)]
pub enum BeaconMethod {
    Genesis,
    Root(StateIdArg),
    FinalityCheckpoints(StateIdArg),
    Validators(ValidatorsArg),
}

#[derive(Args, Debug)]
pub struct StateIdArg {
    #[arg(
        value_parser = clap::value_parser!(StateId),
        long_help = "Identifier for the state under consideration. Possible values are:
    head
    genesis
    finalized
    justified
    <slot>
    <hex-encoded root with 0x prefix>",
    )]
    pub state_id: StateId,
}

#[derive(Args, Debug, Clone)]
pub struct ValidatorsArg {
    pub state_id: StateId,
    pub validator_id: PublicKeyOrIndex,
    pub status: ValidatorStatus,
}
