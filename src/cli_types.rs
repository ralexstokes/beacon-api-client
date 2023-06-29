use crate::{
    api_client::Client,
    types::{BlockId, PeerState, PeerSummary, PublicKeyOrIndex, StateId, ValidatorStatus},
    CommitteeFilter, ConnectionOrientation,
};
use clap::{Args, Parser, Subcommand};
use ethereum_consensus::{networking::PeerId, phase0::BeaconBlockHeader, primitives::Epoch};
use itertools::enumerate;
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
    // #[clap(subcommand)]
    // Events(EventsMethod),
    #[clap(subcommand)]
    Node(NodeMethod),
    #[clap(subcommand)]
    Validator(ValidatorMethod),
}

impl fmt::Display for Namespace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Namespace::Beacon(_) => "beacon",
            Namespace::Config(_) => "config",
            Namespace::Debug(_) => "debug",
            // Namespace::Events(_) => "events",
            Namespace::Node(_) => "node",
            Namespace::Validator(_) => "validator",
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
    ValidatorBalances(ValidatorBalancesArg),
    Committees(CommitteesArg),
    SyncCommittees(SyncCommitteesArg),
    HeaderAtHead(HeaderArg),
    HeaderForSlot(HeaderArg),
    HeaderForParentRoot(HeaderArg),
    HeaderForBlockId(HeaderArg),
    Block(BlockArg),
    //PostBlock(PostBlockArg),
    // PostBlindedBlock,
    BlockRoot(BlockRootArg),
    BlockAttestations(BlockAttestationsArg),
    PoolAttestations(PoolAttestationsArg),
    // PostAttestations,
    AttesterSlashing(AttesterSlashingArg),
    // PostAttesterSlashing,
    ProposerSlashing(ProposerSlashingArg),
    // PostProposerSlashing,
    // PostSyncCommittees,
    VoluntaryExits(VoluntaryExitsArg),
    // PostVoluntaryExits,
}

#[derive(Debug, Clone, Subcommand)]
pub enum ConfigMethod {
    ForkSchedule(ForkScheduleArg),
    Spec(SpecArg),
    DepositContract(DepositContractArg),
}
#[derive(Debug, Clone, Subcommand)]
pub enum DebugMethod {
    //Debug ns
    State(StateArg),
    Head(HeadArg),
}
// #[derive(Debug, Clone, Subcommand)]
// pub enum EventsMethod {
//     //Events ns
//     Events,
// }

#[derive(Debug, Clone, Subcommand)]
pub enum NodeMethod {
    //Node ns
    Identity(IdentityArg),
    Peers(PeersArg),
    Peer(PeerArg),
    PeerSummary(PeerSummaryArg),
    NodeVersion(NodeVersionArg),
    Syncing(SyncingArg),
    Health(HealthArg),
}

#[derive(Debug, Clone, Subcommand)]
pub enum ValidatorMethod {
    //Node ns
    //POST  Attester(AttesterArg),
    ProposerDuties(ProposerDutiesArg),
    //POST Duties(DutiesArg),
    //Blocks(BlocksArg),
}

//ARGS
//BEACON NAMESPACE ARGS
#[derive(Debug, Clone, Args)]
pub struct GenesisArg {
    genesis: Option<StateId>,
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
        // parse filter strings to ValidatorStatus type
        if let Some(f) = &self.filters {
            let s = f.split(",");
            let fil: Vec<&str> = s.collect();
            for i in fil.iter() {
                let j: ValidatorStatus = ValidatorStatus::from_str(i).unwrap();
                filters.push(j.to_owned());
            }
        }
        // call api method
        let out = client
            .get_validators(state_id.to_owned(), &ids.as_slice(), &filters.as_slice())
            .await
            .unwrap();
        println!("{:?}", out);
    }
}

#[derive(Debug, Clone, Args)]
pub struct ValidatorBalancesArg {
    pub state_id: StateId,
    pub filters: Option<String>,
}

impl ValidatorBalancesArg {
    pub async fn execute(&self, client: &Client) {
        let state_id = &self.state_id;
        let mut filters = vec![];
        // parse filter strings to PublicKeyOrIndex type
        if let Some(f) = &self.filters {
            let vec = f.split(",");
            let f_vec: Vec<&str> = vec.collect();
            for i in f_vec.iter() {
                let j: PublicKeyOrIndex = PublicKeyOrIndex::from(i.to_string());
                filters.push(j);
            }
        }
        // call api method
        let out = client.get_balances(state_id.to_owned(), &filters.as_slice()).await.unwrap();
        for i in out {
            println!("Index: {:?}, balance: {:?}", i.index, i.balance)
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct CommitteesArg {
    pub state_id: StateId,
    pub filters: Option<String>,
}

impl CommitteesArg {
    pub async fn execute(&self, client: &Client) {
        let state_id = &self.state_id;

        if let Some(f) = &self.filters {
            let vec = f.split(",");
            let f_vec: Vec<&str> = vec.collect();
            let mut filter = CommitteeFilter { index: None, epoch: None, slot: None };
            if f_vec[0].parse::<u64>().is_ok() {
                filter.index = Some(f_vec[0].parse::<usize>().unwrap());
            }
            if f_vec[1].parse::<usize>().is_ok() {
                filter.epoch = Some(f_vec[1].parse::<u64>().unwrap());
            }
            if f_vec[2].parse::<u64>().is_ok() {
                filter.slot = Some(f_vec[2].parse::<u64>().unwrap());
            }
            let out = client.get_committees(state_id.to_owned(), filter).await.unwrap();
            for i in out {
                println!(
                    "index: {:?}, slot: {:?}, validators: {:?}",
                    i.index, i.slot, i.validators
                );
            }
        } else {
            let out = client.get_all_committees(state_id.to_owned()).await.unwrap();
            for i in out {
                println!(
                    "index: {:?}, slot: {:?}, validators: {:?}",
                    i.index, i.slot, i.validators
                );
            }
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct SyncCommitteesArg {
    pub state_id: StateId,
    pub epoch: Option<Epoch>,
}

impl SyncCommitteesArg {
    pub async fn execute(&self, client: &Client) {
        let id = &self.state_id;
        let mut epoch = None;
        if self.epoch != None {
            epoch = self.epoch;
        }
        let out = client.get_sync_committees(id.to_owned(), epoch).await.unwrap();
        for i in out {
            for j in i.validators {
                println!("{}", &j);
            }
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct HeaderArg {
    arg: Option<String>,
}

impl HeaderArg {
    pub async fn execute(&self, client: &Client) {
        let _out = client.get_beacon_header_at_head().await.unwrap();
        // println!("NOT YET FUNCTIONAL DUE TO ERROR PARSING BLOCK HEADERS IN API CLIENT")
        println!("{:?}", _out.root);
        println!("{:?}", _out.canonical);
        println!("{:?}", _out.signed_header);
    }
}

#[derive(Debug, Clone, Args)]
pub struct BlockArg {
    pub id: BlockId,
}
impl BlockArg {
    pub async fn execute(&self, client: &Client) {
        println!("NOT YET FUNCTIONAL DUE TO ERROR PARSING BLOCK HEADERS IN API CLIENT")
        // let _out = client.get_beacon_block(self.id.to_owned()).await.unwrap();
        // println!("Beacon Block\n");
        // println!("Slot: {:?}\n", _out[0].slot);
        // println!("Proposer index: {:?}\n", _out[0].proposer_index);
        // println!("Parent root: {:?}\n", out.message.parent_root);
        // println!("State root: {:?}\n", out.message.state_root);
        // println!("Body:\n {:?}\n", out.message.body);
        // println!("\n{:?}", out.signature);
    }
}

#[derive(Debug, Clone, Args)]
pub struct BlockRootArg {
    pub id: BlockId,
}
impl BlockRootArg {
    pub async fn execute(&self, client: &Client) {
        let out = client.get_beacon_block_root(self.id.to_owned()).await.unwrap();
        println!("{:?}", out)
    }
}

// #[derive(Debug, Clone, Args)]
// pub struct PostBlockArg {
//     pub block: String,
// }
// impl PostBlockArg{
//     pub fn to_struct(&self) -> Result<SignedBeaconBlock, Error>{
//         println!("\n\ntest 1\n\n\n");
//         println!("{:?}", &self.block);
//         let block_as_value: SignedBeaconBlock = serde_json::from_str(&self.block).unwrap();
//         println!("\n\ntest 2\n\n\n");
//         println!("{:?}", &block_as_value);

//         let signed_beacon_block: SignedBeaconBlock =
// serde_json::from_value(block_as_value).unwrap();         Ok(signed_beacon_block)
//     }
// }

// impl PostBlockArg{
//     pub async fn execute(&self, client: &Client){
//         client.post_signed_beacon_block(&self.to_struct().unwrap()).await.unwrap();
//     }
// }

#[derive(Debug, Clone, Args)]
pub struct BlockAttestationsArg {
    pub id: BlockId,
}

impl BlockAttestationsArg {
    pub async fn execute(&self, client: &Client) {
        let out = client.get_attestations_from_beacon_block(self.id.to_owned()).await.unwrap();
        for i in out {
            println!("{:?}", i);
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct PoolAttestationsArg {
    pub slot: String,
    pub committee_index: String,
}

impl PoolAttestationsArg {
    pub async fn execute(&self, client: &Client) {
        let mut slot = None;
        let mut committee_index = None;
        if self.slot != "None" {
            slot = Some(self.slot.parse::<u64>().unwrap());
        }
        if self.committee_index != "None" {
            committee_index = Some(self.committee_index.parse::<usize>().unwrap());
        }
        let out = client.get_attestations_from_pool(slot, committee_index).await.unwrap();
        for i in out {
            println!("{:?}", i);
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct AttesterSlashingArg {
    pub arg: Option<String>,
}

impl AttesterSlashingArg {
    pub async fn execute(&self, client: &Client) {
        let result = client.get_attester_slashings_from_pool().await.unwrap();
        for i in result {
            println!("{:?}", i);
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct ProposerSlashingArg {
    pub arg: Option<String>,
}

impl ProposerSlashingArg {
    pub async fn execute(&self, client: &Client) {
        let result = client.get_proposer_slashings_from_pool().await.unwrap();
        for i in result {
            println!("{:?}", i);
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct VoluntaryExitsArg {
    pub arg: Option<String>,
}

impl VoluntaryExitsArg {
    pub async fn execute(&self, client: &Client) {
        let result = client.get_voluntary_exits_from_pool().await.unwrap();
        for i in result {
            println!("{:?}", i);
        }
    }
}

// CONFIG NAMESPACE ARGS

#[derive(Debug, Clone, Args)]
pub struct ForkScheduleArg {
    pub arg: Option<String>,
}

impl ForkScheduleArg {
    pub async fn execute(&self, client: &Client) {
        let result = client.get_fork_schedule().await.unwrap();
        println!("{:?}", result)
    }
}

#[derive(Debug, Clone, Args)]
pub struct SpecArg {
    pub arg: Option<String>,
}

impl SpecArg {
    pub async fn execute(&self, client: &Client) {
        let result = client.get_spec().await.unwrap();
        println!("{:?}", result)
    }
}

#[derive(Debug, Clone, Args)]
pub struct DepositContractArg {
    pub arg: Option<String>,
}

impl DepositContractArg {
    pub async fn execute(&self, client: &Client) {
        let result = client.get_deposit_contract_address().await.unwrap();
        println!("{:?}", result.address)
    }
}

// DEBUG NAMESACE ARGS

#[derive(Debug, Clone, Args)]
pub struct StateArg {
    state_id: StateId,
}

impl StateArg {
    pub async fn execute(&self, client: &Client) {
        let id = &self.state_id;
        let result = client.get_state(id.to_owned()).await.unwrap();
        println!("{:?}", result)
    }
}

#[derive(Debug, Clone, Args)]
pub struct HeadArg {
    pub arg: Option<String>,
}

impl HeadArg {
    pub async fn execute(&self, client: &Client) {
        let result = client.get_heads().await.unwrap();
        // uncomment when this PR is merged https://github.com/ralexstokes/ethereum-consensus/pull/196
        // for i in result.iter() {
        //     println!("{:?}", i.coordinate)
        // }
    }
}

//Node ns args

#[derive(Debug, Clone, Args)]
pub struct IdentityArg {
    pub arg: Option<String>,
}

impl IdentityArg {
    pub async fn execute(&self, client: &Client) {
        let result = client.get_node_identity().await.unwrap();
        println!("Discovery Addresses:");
        for i in result.discovery_addresses.iter() {
            println!("{}", i);
        }
        println!("P2P Addresses:");
        for i in result.p2p_addresses.iter() {
            println!("{}", i);
        }
        println!("Peer ID: {}", result.peer_id);
        println!("ENR: {}", result.enr);
        println!("Metadata: {:?}", result.metadata)
    }
}

#[derive(Debug, Clone, Args)]
pub struct PeersArg {
    pub peer_state: String,
    pub orientation: String,
}

impl PeersArg {
    pub async fn execute(&self, client: &Client) {
        let p = &self.peer_state;
        let p_vec = p.split(",");
        let peer_vec: Vec<&str> = p_vec.collect();
        let mut peer_states = vec![];
        for i in peer_vec.iter() {
            let state: PeerState = PeerState::from_str(i.trim_start().trim_end()).unwrap();
            peer_states.push(state)
        }
        let o = &self.orientation;
        let o_vec = o.split(",");
        let orientation_vec: Vec<&str> = o_vec.collect();
        let mut orientations = vec![];
        for i in orientation_vec.iter() {
            let orientation: ConnectionOrientation = ConnectionOrientation::from_str(i).unwrap();
            orientations.push(orientation)
        }
        let out =
            client.get_node_peers(peer_states.as_slice(), orientations.as_slice()).await.unwrap();

        println!("length of output: {:?}", out.len());
        for (i, data) in enumerate(out) {
            println!("\nPeer {}", i);
            println!("id: {}", &data.peer_id);
            println!("ENR: {:?}", &data.enr);
            println!("Last seen p2p address: {}", &data.last_seen_p2p_address);
            println!("state: {}", &data.state);
            println!("connection orientation: {}", &data.direction);
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct PeerArg {
    pub peer_id: String,
}

impl PeerArg {
    pub async fn execute(&self, client: &Client) {
        let id: PeerId = PeerId::from_str(&self.peer_id).unwrap();
        let out = client.get_peer(id).await.unwrap();
        println!("id: {}", &out.peer_id);
        println!("ENR: {:?}", &out.enr);
        println!("Last seen p2p address: {}", &out.last_seen_p2p_address);
        println!("state: {}", &out.state);
        println!("connection orientation: {}", &out.direction);
    }
}

#[derive(Debug, Clone, Args)]
pub struct PeerSummaryArg {
    pub arg: Option<String>,
}

impl PeerSummaryArg {
    pub async fn execute(&self, client: &Client) {
        let out = client.get_peer_summary().await.unwrap();
        println!("disconnected: {}", &out.disconnected);
        println!("connecting: {}", &out.connecting);
        println!("connected: {}", &out.connected);
        println!("disconnecting: {}", &out.disconnecting);
    }
}

#[derive(Debug, Clone, Args)]
pub struct NodeVersionArg {
    pub arg: Option<String>,
}

impl NodeVersionArg {
    pub async fn execute(&self, client: &Client) {
        let out = client.get_node_version().await.unwrap();
        println!("{}", &out);
    }
}

#[derive(Debug, Clone, Args)]
pub struct SyncingArg {
    pub arg: Option<String>,
}

impl SyncingArg {
    pub async fn execute(&self, client: &Client) {
        let out = client.get_sync_status().await.unwrap();
        println!("head slot: {}", &out.head_slot);
        println!("syncing distance: {}", &out.sync_distance);
        println!("is syncing?: {}", &out.is_syncing);
    }
}

#[derive(Debug, Clone, Args)]
pub struct HealthArg {
    pub arg: Option<String>,
}

impl HealthArg {
    pub async fn execute(&self, client: &Client) {
        let out = client.get_health().await.unwrap();
        println!("health status: {:?}", out);
    }
}

//validator ns args

#[derive(Debug, Clone, Args)]
pub struct ProposerDutiesArg {
    pub epoch: String,
}

impl ProposerDutiesArg {
    pub async fn execute(&self, client: &Client) {
        let epoch: Epoch = self.epoch.parse::<u64>().unwrap();
        let out = client.get_proposer_duties(epoch).await.unwrap();
        println!("health status: {:?}", out);
    }
}
