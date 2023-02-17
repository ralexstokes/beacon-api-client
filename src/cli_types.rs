use crate::{
    api_client::Client,
    types::{PublicKeyOrIndex, StateId, ValidatorStatus},
};
use clap::{Args, Parser, Subcommand};
use ethereum_consensus::primitives::BlsPublicKey;
use std::{fmt, str::FromStr};

#[derive(Debug, Parser)]
#[clap(version, about = "Beacon API client")]
pub struct CliConfig {
    #[clap(short, long)]
    pub endpoint: String,
    #[clap(subcommand)]
    pub command: Namespace,
}

#[derive(Debug, Clone, Subcommand)]
#[clap(author, version, about)]
pub enum Namespace {
    #[clap(subcommand)]
    Beacon(BeaconMethod),
    #[clap(subcommand)]
    Config(ConfigMethod),
    #[clap(subcommand)]
    Debug(DebugMethod),
    #[clap(subcommand)]
    Events(EventsMethod),
    // Node(NodeMethod),
    // Validator(ValidatorMethod),
}

impl fmt::Display for Namespace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Namespace::Beacon(_) => "beacon",
            Namespace::Config(_) => "config",
            Namespace::Debug(_) => "debug",
            Namespace::Events(_) => "events",
            // Namespace::Node(_) => "node",
            // Namespace::Validator(_) => "validator",
        };
        write!(f, "{printable}")
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum BeaconMethod {
    //Beacon ns
    Genesis(GenesisArg),
    Root(RootArg),
    Fork(ForkArg),
    FinalityCheckpoints(FinalityCheckpointsArg),
    Validator(ValidatorArg),
    Validators(ValidatorsArg),
    // ValidatorBalances(ValidatorBalancesArg),
    // Committees(CommitteesArg),
    // SyncCommittees(SyncCommitteesArg),
    // HeaderAtHead,
    // HeaderForSlot,
    // HeaderForParentRoot,
    // HeaderForBockId,
    // Block,
    // PostBlock,
    // PostBlindedBlock,
    // BlockRoot,
    // BlockAttestations,
    // PoolAttestations,
    // PostAttestations,
    // AttesterSlashing,
    // PostAttesterSlashing,
    // ProposerSlashing,
    // PostProposerSlashing,
    // PostSyncCommittees,
    // VoluntaryExits,
    // PostVoluntaryExits,
}

#[derive(Debug, Clone, Subcommand)]
pub enum ConfigMethod {
    ForkSchedule,
    Spec,
    DepositContract,
}
#[derive(Debug, Clone, Subcommand)]
pub enum DebugMethod {
    //Debug ns
    State,
    Head,
}
#[derive(Debug, Clone, Subcommand)]
pub enum EventsMethod {
    //Events ns
    Events,
}

// arguments for each Namespace::Method subcommand
#[derive(Debug, Clone, Args)]
pub struct GenesisArg {
    genesis: Option<String>,
}

impl GenesisArg {
    pub async fn execute(&self, client: &Client) {
        let out = client.get_genesis_details().await.unwrap();
        println!("{:?}", out);
    }
}

#[derive(Debug, Clone, Args)]
pub struct RootArg {
    pub state_id: StateId,
}

impl RootArg {
    pub async fn execute(&self, client: &Client) {
        let id = &self.state_id;
        let out = client.get_state_root(id.to_owned()).await.unwrap();
        println!("{:?}", out);
    }
}

#[derive(Debug, Clone, Args)]
pub struct ForkArg {
    pub state_id: StateId,
}

impl ForkArg {
    pub async fn execute(&self, client: &Client) {
        let id = &self.state_id;
        let out = client.get_fork(id.to_owned()).await.unwrap();
        println!("{:?}", out);
    }
}

#[derive(Debug, Clone, Args)]
pub struct FinalityCheckpointsArg {
    pub state_id: StateId,
}

impl FinalityCheckpointsArg {
    pub async fn execute(&self, client: &Client) {
        let id = &self.state_id;
        let out = client.get_finality_checkpoints(id.to_owned()).await.unwrap();
        println!("current justified: {:?}", out.current_justified);
        println!("finalized: {:?}", out.finalized);
    }
}

#[derive(Debug, Clone, Args)]
pub struct ValidatorArg {
    pub state_id: StateId,
    pub validator_id: PublicKeyOrIndex,
}
impl ValidatorArg {
    pub async fn execute(&self, client: &Client) {
        let id = &self.state_id;
        let validator_id = &self.validator_id;
        let out = client.get_validator(id.to_owned(), validator_id.to_owned()).await.unwrap();
        println!("{:?}", out);
    }
}

#[derive(Debug, Clone, Args)]
pub struct ValidatorsArg {
    pub state_id: StateId,
    pub validator_ids: Option<String>,
    pub filters: Option<String>,
}

impl ValidatorsArg {
    pub async fn execute(&self, client: &Client) {
        let state_id = &self.state_id;
        // parse validator_id strings to PublicKeyOrIndex type
        let mut ids = vec![];
        let mut filters = vec![];
        if let Some(id) = &self.validator_ids {
            let vec = id.split(",");
            let id_vec: Vec<&str> = vec.collect();
            for i in id_vec.iter() {
                let j: PublicKeyOrIndex = PublicKeyOrIndex::from(i.to_string());
                ids.push(j);
            }
        }
        // parse filter strings to PublicKeyOrIndex type
        if let Some(f) = &self.filters {
            let s = f.split(",");
            let fil: Vec<&str> = s.collect();
            for i in fil.iter() {
                let j: ValidatorStatus = ValidatorStatus::from_str(i).unwrap();
                filters.push(j.to_owned());
            }
        }
        let out = client
            .get_validators(state_id.to_owned(), &ids.as_slice(), &filters.as_slice())
            .await
            .unwrap();
        println!("{:?}", out);
    }
}


#[derive(Debug, Clone, Args)]
pub struct ValidatorBalancesArg {
    pub state_id: String,
    pub filters: String,
}

#[derive(Debug, Clone, Args)]
pub struct CommitteesArg {
    pub state_id: String,
    pub filters: String,
}

#[derive(Debug, Clone, Args)]
pub struct SyncCommitteesArg {
    pub state_id: String,
    pub epoch: String,
}

#[derive(Debug, Clone, Args)]
pub struct HeaderArg {
    pub slot: String,
}
