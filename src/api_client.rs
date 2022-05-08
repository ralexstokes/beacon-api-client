// use ethereum_consensus::altair::mainnet::{
//     SignedContributionAndProof, SyncCommitteeContribution, SyncCommitteeMessage,
// };
// use ethereum_consensus::bellatrix::mainnet::{BlindedBeaconBlock, SignedBlindedBeaconBlock};
// use ethereum_consensus::networking::{Enr, MetaData, Multiaddr, PeerId};
// use ethereum_consensus::phase0::mainnet::{
//     Attestation, AttestationData, AttesterSlashing, BeaconBlock, BeaconState, Checkpoint, Fork,
//     ProposerSlashing, SignedAggregateAndProof, SignedBeaconBlock, SignedBeaconBlockHeader,
//     SignedVoluntaryExit, Validator,
// };
// use ethereum_consensus::primitives::{
//     BlsPublicKey, Bytes32, ChainId, CommitteeIndex, Coordinate, Epoch, ExecutionAddress, Gwei,
//     RandaoReveal, Root, Slot, ValidatorIndex, Version,
// };
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub enum Error {}

#[derive(Serialize, Deserialize)]
pub struct Client {

}


#[tokio::main]
pub async fn http_request(
    ip_addr: &str,
    endpoint: &str,
    state_id: &str,
    port_id: &str
) -> Result<serde_json::Value, reqwest::Error> {
    
    let url: String = format!("http://{}:{}/{}/{}", ip_addr, port_id, endpoint, state_id);

    println!("{:?}", &url);

    let client = reqwest::Client::new();

    let response = client.get(&url).send().await?;

    let body = response.text().await.unwrap();

    let result: serde_json::Value =
        serde_json::from_str(&body).expect("JSON was not well-formatted");

    Ok(result)
}




// #[derive(Serialize, Deserialize)]
// pub struct GenesisDetails {
//     #[serde(with = "crate::serde::as_string")]
//     pub genesis_time: u64,
//     pub genesis_validators_root: Root,
//     #[serde(with = "crate::serde::as_hex")]
//     pub genesis_fork_version: Version,
// }

// #[derive(Serialize, Deserialize)]
// pub enum StateId {
//     Head,
//     Genesis,
//     Finalized,
//     Justified,
//     Slot(Slot),
//     Root(Root),
// }

// #[derive(Serialize, Deserialize)]
// pub enum BlockId {
//     Head,
//     Genesis,
//     Finalized,
//     Slot(Slot),
//     Root(Root),
// }

// #[derive(Serialize, Deserialize)]
// pub enum ExecutionStatus {
//     Default,
//     Optimistic,
// }

// #[derive(Serialize, Deserialize)]
// pub struct FinalityCheckpoints {
//     pub previous_justified: Checkpoint,
//     pub current_justified: Checkpoint,
//     pub finalized: Checkpoint,
// }

// #[derive(Serialize, Deserialize)]
// pub enum ValidatorStatus {
//     PendingInitialized,
//     PendingQueued,
//     ActiveOngoing,
//     ActiveExiting,
//     ActiveSlashed,
//     ExitedUnslashed,
//     ExitedSlashed,
//     WithdrawalPossible,
//     WithdrawalDone,
//     // TODO what are these?
//     Active,
//     Pending,
//     Exited,
//     Withdrawal,
// }

// pub enum PubkeyOrIndex {
//     Pubkey(BlsPublicKey),
//     Index(ValidatorIndex),
// }

// pub struct ValidatorDescriptor {
//     pub pubkey_or_index: PubkeyOrIndex,
//     pub status: ValidatorStatus,
// }

// #[derive(Serialize, Deserialize)]
// pub struct ValidatorSummary {
//     #[serde(with = "crate::serde::as_string")]
//     pub index: ValidatorIndex,
//     #[serde(with = "crate::serde::as_string")]
//     pub balance: Gwei,
//     pub status: ValidatorStatus,
//     pub validator: Validator,
// }

// #[derive(Serialize, Deserialize)]
// pub struct BalanceSummary {
//     #[serde(with = "crate::serde::as_string")]
//     pub index: ValidatorIndex,
//     #[serde(with = "crate::serde::as_string")]
//     pub balance: Gwei,
// }

// pub struct CommitteeFilter {
//     pub epoch: Option<Epoch>,
//     pub index: Option<CommitteeIndex>,
//     pub slot: Option<Slot>,
// }

// #[derive(Serialize, Deserialize)]
// pub struct CommitteeSummary {
//     #[serde(with = "crate::serde::as_string")]
//     pub index: CommitteeIndex,
//     #[serde(with = "crate::serde::as_string")]
//     pub slot: Slot,
//     #[serde(with = "crate::serde::collection_over_string")]
//     pub validators: Vec<ValidatorIndex>,
// }

// #[derive(Serialize, Deserialize)]
// pub struct SyncCommitteeSummary {
//     #[serde(with = "crate::serde::collection_over_string")]
//     pub validators: Vec<ValidatorIndex>,
//     // TODO fix serde here
//     pub validator_aggregates: Vec<Vec<ValidatorIndex>>,
// }

// #[derive(Serialize, Deserialize)]
// pub struct BeaconHeaderSummary {
//     pub root: Root,
//     pub canonical: bool,
//     pub signed_header: SignedBeaconBlockHeader,
// }

// pub enum EventTopic {
//     Head,
//     Block,
//     Attestation,
//     VoluntaryExit,
//     FinalizedCheckpoint,
//     ChainReorg,
//     ContributionAndProof,
// }

// #[derive(Serialize, Deserialize)]
// pub struct NetworkIdentity {
//     pub peer_id: PeerId,
//     pub enr: Enr,
//     pub p2p_addresses: Vec<Multiaddr>,
//     pub discovery_addresses: Vec<Multiaddr>,
//     pub metadata: MetaData,
// }

// #[derive(Serialize, Deserialize)]
// pub enum PeerState {
//     Disconnected,
//     Connecting,
//     Connected,
//     Disconnecting,
// }

// #[derive(Serialize, Deserialize)]
// pub enum ConnectionOrientation {
//     Inbound,
//     Outbound,
// }

// #[derive(Serialize, Deserialize)]
// pub struct PeerDescriptor {
//     pub state: PeerState,
//     pub direction: ConnectionOrientation,
// }

// #[derive(Serialize, Deserialize)]
// pub struct PeerDescription {
//     pub peer_id: PeerId,
//     pub enr: Enr,
//     pub last_seen_p2p_address: Multiaddr,
//     pub state: PeerState,
//     pub direction: ConnectionOrientation,
// }

// #[derive(Serialize, Deserialize)]
// pub struct PeerSummary {
//     #[serde(with = "crate::serde::as_string")]
//     pub disconnected: usize,
//     #[serde(with = "crate::serde::as_string")]
//     pub connecting: usize,
//     #[serde(with = "crate::serde::as_string")]
//     pub connected: usize,
//     #[serde(with = "crate::serde::as_string")]
//     pub disconnecting: usize,
// }

// #[derive(Serialize, Deserialize)]
// pub struct SyncStatus {
//     #[serde(with = "crate::serde::as_string")]
//     pub head_slot: Slot,
//     #[serde(with = "crate::serde::as_string")]
//     pub sync_distance: usize,
//     pub is_syncing: bool,
// }

// #[derive(Serialize, Deserialize)]
// pub enum HealthStatus {
//     Ready,
//     Syncing,
//     NotInitialized,
// }

// #[derive(Serialize, Deserialize)]
// pub struct AttestationDuty {
//     pub pubkey: BlsPublicKey,
//     #[serde(with = "crate::serde::as_string")]
//     pub validator_index: ValidatorIndex,
//     #[serde(with = "crate::serde::as_string")]
//     pub committee_index: CommitteeIndex,
//     #[serde(with = "crate::serde::as_string")]
//     pub committee_length: usize,
//     #[serde(with = "crate::serde::as_string")]
//     pub committees_at_slot: usize,
//     #[serde(with = "crate::serde::as_string")]
//     pub validator_committee_index: usize,
//     #[serde(with = "crate::serde::as_string")]
//     pub slot: Slot,
// }

// #[derive(Serialize, Deserialize)]
// pub struct ProposerDuty {
//     pub pubkey: BlsPublicKey,
//     #[serde(with = "crate::serde::as_string")]
//     pub validator_index: ValidatorIndex,
//     #[serde(with = "crate::serde::as_string")]
//     pub slot: Slot,
// }

// #[derive(Serialize, Deserialize)]
// pub struct SyncCommitteeDuty {
//     pub pubkey: BlsPublicKey,
//     #[serde(with = "crate::serde::as_string")]
//     pub validator_index: ValidatorIndex,
//     #[serde(with = "crate::serde::collection_over_string")]
//     pub validator_sync_committee_indices: Vec<usize>,
// }

// pub struct CommitteeDescriptor {
//     pub validator_index: ValidatorIndex,
//     pub committee_index: CommitteeIndex,
//     pub committees_at_slot: usize,
//     pub slot: Slot,
//     pub is_aggregator: bool,
// }

// pub struct SyncCommitteeDescriptor {
//     pub validator_index: ValidatorIndex,
//     pub sync_committee_indices: Vec<usize>,
//     pub until_epoch: Epoch,
// }

// #[derive(Serialize, Deserialize)]
// pub struct BeaconProposerRegistration {
//     #[serde(with = "crate::serde::as_string")]
//     pub validator_index: ValidatorIndex,
//     pub fee_recipient: ExecutionAddress,
// }

// impl Client {
//     /* beacon namespace */
//     pub async fn get_genesis_details(&self) -> Result<GenesisDetails, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_state_root(id: StateId) -> Result<Root, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_fork(id: StateId) -> Result<Fork, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_finality_checkpoints(id: StateId) -> Result<FinalityCheckpoints, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_validators(
//         id: StateId,
//         filters: &[ValidatorDescriptor],
//     ) -> Result<Vec<ValidatorSummary>, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_validator(
//         id: StateId,
//         validator_id: PubkeyOrIndex,
//     ) -> Result<ValidatorSummary, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_balances(
//         id: StateId,
//         filters: &[PubkeyOrIndex],
//     ) -> Result<Vec<BalanceSummary>, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_all_committees(id: StateId) -> Result<Vec<CommitteeSummary>, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_committees(
//         id: StateId,
//         filter: CommitteeFilter,
//     ) -> Result<Vec<CommitteeSummary>, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_sync_committees(
//         id: StateId,
//         epoch: Option<Epoch>,
//     ) -> Result<Vec<SyncCommitteeSummary>, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_beacon_header_at_head() -> Result<BeaconHeaderSummary, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_beacon_header_for_slot(slot: Slot) -> Result<BeaconHeaderSummary, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_beacon_header_for_parent_root(
//         parent_root: Root,
//     ) -> Result<BeaconHeaderSummary, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_beacon_header(id: BlockId) -> Result<BeaconHeaderSummary, Error> {
//         unimplemented!("")
//     }

//     pub async fn post_signed_beacon_block(block: &SignedBeaconBlock) -> Result<(), Error> {
//         unimplemented!("")
//     }

//     pub async fn post_signed_blinded_beacon_block(
//         block: &SignedBlindedBeaconBlock,
//     ) -> Result<(), Error> {
//         unimplemented!("")
//     }

//     // v2 endpoint
//     pub async fn get_beacon_block(id: BlockId) -> Result<SignedBeaconBlock, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_beacon_block_root(id: BlockId) -> Result<Root, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_attestations_from_beacon_block(
//         id: BlockId,
//     ) -> Result<Vec<Attestation>, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_attestations_from_pool(
//         slot: Option<Slot>,
//         committee_index: Option<CommitteeIndex>,
//     ) -> Result<Vec<Attestation>, Error> {
//         unimplemented!("")
//     }

//     pub async fn post_attestations(attestations: &[Attestation]) -> Result<(), Error> {
//         unimplemented!("")
//     }

//     pub async fn get_attester_slashings_from_pool() -> Result<Vec<AttesterSlashing>, Error> {
//         unimplemented!("")
//     }

//     pub async fn post_attester_slashing(attester_slashing: &AttesterSlashing) -> Result<(), Error> {
//         unimplemented!("")
//     }

//     pub async fn get_proposer_slashings_from_pool() -> Result<Vec<ProposerSlashing>, Error> {
//         unimplemented!("")
//     }

//     pub async fn post_proposer_slashing(proposer_slashing: &ProposerSlashing) -> Result<(), Error> {
//         unimplemented!("")
//     }

//     pub async fn post_sync_committee_messages(
//         messages: &[SyncCommitteeMessage],
//     ) -> Result<(), Error> {
//         unimplemented!("")
//     }

//     pub async fn get_voluntary_exits_from_pool() -> Result<Vec<SignedVoluntaryExit>, Error> {
//         unimplemented!("")
//     }

//     pub async fn post_signed_voluntary_exit(exit: &SignedVoluntaryExit) -> Result<(), Error> {
//         unimplemented!("")
//     }

//     /* config namespace */
//     pub async fn get_fork_schedule() -> Result<Vec<Fork>, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_spec() -> Result<HashMap<String, String>, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_deposit_contract_address() -> Result<(ChainId, ExecutionAddress), Error> {
//         unimplemented!("")
//     }

//     /* debug namespace */
//     // v2 endpoint
//     pub async fn get_state(id: StateId) -> Result<BeaconState, Error> {
//         unimplemented!("")
//     }

//     // v2 endpoint
//     pub async fn get_heads() -> Result<Vec<Coordinate>, Error> {
//         unimplemented!("")
//     }

//     /* events namespace */
//     // TODO: figure out return type
//     pub async fn get_events<T>(topics: &[EventTopic]) -> Result<T, Error> {
//         // get back "event: TOPIC, data: T"
//         unimplemented!("")
//     }

//     /* node namespace */
//     pub async fn get_node_identity() -> Result<NetworkIdentity, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_node_peers(filters: &[PeerDescriptor]) -> Result<Vec<PeerDescription>, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_peer(peer_id: Multiaddr) -> Result<PeerDescription, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_peer_count() -> Result<PeerSummary, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_node_version() -> Result<String, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_sync_status() -> Result<SyncStatus, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_health() -> Result<HealthStatus, Error> {
//         unimplemented!("")
//     }

//     /* validator namespace */
//     pub async fn get_attester_duties(
//         epoch: Epoch,
//         indices: &[ValidatorIndex],
//     ) -> Result<(Root, Vec<AttestationDuty>), Error> {
//         unimplemented!("")
//     }

//     pub async fn get_proposer_duties(epoch: Epoch) -> Result<(Root, Vec<ProposerDuty>), Error> {
//         unimplemented!("")
//     }

//     pub async fn get_sync_committee_duties(
//         epoch: Epoch,
//         indices: &[ValidatorIndex],
//     ) -> Result<(Root, Vec<SyncCommitteeDuty>), Error> {
//         unimplemented!("")
//     }

//     // v2 endpoint
//     pub async fn get_block(
//         slot: Slot,
//         randao_reveal: RandaoReveal,
//         graffiti: Bytes32,
//     ) -> Result<BeaconBlock, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_blinded_block(
//         slot: Slot,
//         randao_reveal: RandaoReveal,
//         graffiti: Bytes32,
//     ) -> Result<BlindedBeaconBlock, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_attestation_data(
//         slot: Slot,
//         committee_index: CommitteeIndex,
//     ) -> Result<AttestationData, Error> {
//         unimplemented!("")
//     }

//     pub async fn get_attestation_aggregate(
//         attestation_data_root: Root,
//         slot: Slot,
//     ) -> Result<Attestation, Error> {
//         unimplemented!("")
//     }

//     pub async fn post_aggregates_with_proofs(
//         aggregates_with_proofs: &[SignedAggregateAndProof],
//     ) -> Result<(), Error> {
//         Ok(())
//     }

//     pub async fn subscribe_subnets_for_attestation(
//         committee_descriptors: &[CommitteeDescriptor],
//     ) -> Result<(), Error> {
//         Ok(())
//     }

//     pub async fn subscribe_subnets_for_sync_committee(
//         sync_committee_descriptors: &[SyncCommitteeDescriptor],
//     ) -> Result<(), Error> {
//         Ok(())
//     }

//     pub async fn get_sync_committee_contribution(
//         slot: Slot,
//         subcommittee_index: usize,
//         beacon_block_root: Root,
//     ) -> Result<SyncCommitteeContribution, Error> {
//         unimplemented!("")
//     }

//     pub async fn post_contributions_with_proofs(
//         contributions_with_proofs: &[SignedContributionAndProof],
//     ) -> Result<(), Error> {
//         Ok(())
//     }

//     pub async fn register_proposers(
//         registrations: &[BeaconProposerRegistration],
//     ) -> Result<(), Error> {
//         Ok(())
//     }
// }
