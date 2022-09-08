// TODO: remove once things are all used
#![allow(unused_variables)]

use crate::error::ApiError;
use crate::types::{
    ApiResult, AttestationDuty, BalanceSummary, BeaconHeaderSummary, BeaconHeaderSummaryOuter,
    BeaconProposerRegistration, BlockId, CommitteeDescriptor, CommitteeFilter, CommitteeSummary,
    EventTopic, FinalityCheckpoints, GenesisDetails, HealthStatus, PeerSummary, ProposerDuty,
    PublicKeyOrIndex, RootData, StateId, SyncCommitteeDescriptor, SyncCommitteeDuty,
    SyncCommitteeSummary, SyncStatus, ValidatorStatus, ValidatorSummary, Value, VersionData,
};
#[cfg(feature = "peer-id")]
use crate::types::{NetworkIdentity, PeerDescription, PeerDescriptor};
use ethereum_consensus::altair::mainnet::{
    SignedContributionAndProof, SyncCommitteeContribution, SyncCommitteeMessage,
};
use ethereum_consensus::bellatrix::mainnet::{BlindedBeaconBlock, SignedBlindedBeaconBlock};
use ethereum_consensus::builder::SignedValidatorRegistration;
#[cfg(feature = "peer-id")]
use ethereum_consensus::networking::Multiaddr;
use ethereum_consensus::phase0::mainnet::{
    Attestation, AttestationData, AttesterSlashing, BeaconBlock, BeaconState, Fork,
    ProposerSlashing, SignedAggregateAndProof, SignedBeaconBlock, SignedVoluntaryExit,
};
use ethereum_consensus::primitives::{
    Bytes32, ChainId, CommitteeIndex, Coordinate, Epoch, ExecutionAddress, RandaoReveal, Root,
    Slot, ValidatorIndex,
};
use http::StatusCode;
use itertools::Itertools;
use std::collections::HashMap;
use thiserror::Error;
use url::{ParseError, Url};

#[derive(Debug, Error)]
pub enum Error {
    #[error("could not parse URL: {0}")]
    Url(#[from] ParseError),
    #[error("could not send request: {0}")]
    Http(#[from] reqwest::Error),
    #[error("error from API: {0}")]
    Api(#[from] ApiError),
    #[error("missing expected data in response: {0}")]
    MissingExpectedData(String),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

pub async fn api_error_or_ok(response: reqwest::Response) -> Result<(), Error> {
    if response.status() == reqwest::StatusCode::OK {
        Ok(())
    } else {
        let api_err = response.json::<ApiError>().await?;
        Err(Error::Api(api_err))
    }
}

#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    endpoint: Url,
}

impl Client {
    pub fn new_with_client<U: Into<Url>>(client: reqwest::Client, endpoint: U) -> Self {
        Self {
            http: client,
            endpoint: endpoint.into(),
        }
    }

    pub fn new<U: Into<Url>>(endpoint: U) -> Self {
        let client = reqwest::Client::new();
        Self::new_with_client(client, endpoint)
    }

    pub async fn get<T: serde::Serialize + serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, Error> {
        let result: ApiResult<T> = self.http_get(path).await?.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn http_get(&self, path: &str) -> Result<reqwest::Response, Error> {
        let target = self.endpoint.join(path)?;
        let response = self.http.get(target).send().await?;
        Ok(response)
    }

    pub async fn post<T: serde::Serialize + ?Sized>(
        &self,
        path: &str,
        argument: &T,
    ) -> Result<(), Error> {
        let response = self.http_post(path, argument).await?;
        api_error_or_ok(response).await
    }

    pub async fn http_post<T: serde::Serialize + ?Sized>(
        &self,
        path: &str,
        argument: &T,
    ) -> Result<reqwest::Response, Error> {
        let target = self.endpoint.join(path)?;
        let response = self.http.post(target).json(argument).send().await?;
        Ok(response)
    }

    /* beacon namespace */
    pub async fn get_genesis_details(&self) -> Result<GenesisDetails, Error> {
        let details: Value<GenesisDetails> = self.get("/eth/v1/beacon/genesis").await?;
        Ok(details.data)
    }

    pub async fn get_state_root(&self, state_id: StateId) -> Result<Root, Error> {
        let path = format!("eth/v1/beacon/states/{state_id}/root");
        let root: Value<RootData> = self.get(&path).await?;
        Ok(root.data.root)
    }

    pub async fn get_fork(&self, state_id: StateId) -> Result<Fork, Error> {
        let path = format!("eth/v1/beacon/states/{state_id}/fork");
        let result: Value<Fork> = self.get(&path).await?;
        Ok(result.data)
    }

    pub async fn get_finality_checkpoints(
        &self,
        id: StateId,
    ) -> Result<FinalityCheckpoints, Error> {
        let path = format!("eth/v1/beacon/states/{id}/finality_checkpoints");
        let result: Value<FinalityCheckpoints> = self.get(&path).await?;
        Ok(result.data)
    }

    pub async fn get_validators(
        &self,
        state_id: StateId,
        validator_ids: &[PublicKeyOrIndex],
        filters: &[ValidatorStatus],
    ) -> Result<Vec<ValidatorSummary>, Error> {
        let path = format!("eth/v1/beacon/states/{state_id}/validators");
        let target = self.endpoint.join(&path)?;
        let mut request = self.http.get(target);
        if !validator_ids.is_empty() {
            let validator_ids = validator_ids.iter().join(", ");
            request = request.query(&[("id", validator_ids)]);
        }
        if !filters.is_empty() {
            let filters = filters.iter().join(", ");
            request = request.query(&[("status", filters)]);
        }
        let response = request.send().await?;

        let result: ApiResult<Value<Vec<ValidatorSummary>>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn get_validator(
        &self,
        state_id: StateId,
        validator_id: PublicKeyOrIndex,
    ) -> Result<ValidatorSummary, Error> {
        let path = format!("eth/v1/beacon/states/{state_id}/validators/{validator_id}");
        let result: Value<ValidatorSummary> = self.get(&path).await?;
        Ok(result.data)
    }

    pub async fn get_balances(
        &self,
        id: StateId,
        filters: &[PublicKeyOrIndex],
    ) -> Result<Vec<BalanceSummary>, Error> {
        let path = format!("eth/v1/beacon/states/{id}/validator_balances");
        let target = self.endpoint.join(&path)?;
        let mut request = self.http.get(target);

        if !filters.is_empty() {
            let filters = filters.iter().join(", ");
            request = request.query(&[("id", filters)]);
        }
        let response = request.send().await?;

        let result: ApiResult<Value<Vec<BalanceSummary>>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn get_all_committees(&self, id: StateId) -> Result<Vec<CommitteeSummary>, Error> {
        self.get_committees(id, CommitteeFilter::default()).await
    }

    pub async fn get_committees(
        &self,
        id: StateId,
        filter: CommitteeFilter,
    ) -> Result<Vec<CommitteeSummary>, Error> {
        let path = format!("eth/v1/beacon/states/{id}/committees");
        let target = self.endpoint.join(&path)?;
        let mut request = self.http.get(target);
        if let Some(epoch) = filter.epoch {
            request = request.query(&[("epoch", epoch)]);
        }
        if let Some(index) = filter.index {
            request = request.query(&[("index", index)]);
        }
        if let Some(slot) = filter.slot {
            request = request.query(&[("slot", slot)]);
        }
        let response = request.send().await?;
        let result: ApiResult<Value<Vec<CommitteeSummary>>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn get_sync_committees(
        &self,
        id: StateId,
        epoch: Option<Epoch>,
    ) -> Result<Vec<SyncCommitteeSummary>, Error> {
        unimplemented!("")
    }

    pub async fn get_beacon_header_at_head(&self) -> Result<BeaconHeaderSummaryOuter, Error> {
        let result: BeaconHeaderSummaryOuter = self.get(&"eth/v1/beacon/headers/head").await?;
        Ok(result)
    }

    pub async fn get_beacon_header_for_slot(
        &self,
        slot: Slot,
    ) -> Result<BeaconHeaderSummary, Error> {
        unimplemented!("")
    }

    pub async fn get_beacon_header_for_parent_root(
        parent_root: Root,
    ) -> Result<BeaconHeaderSummary, Error> {
        unimplemented!("")
    }

    pub async fn get_beacon_header(id: BlockId) -> Result<BeaconHeaderSummary, Error> {
        unimplemented!("")
    }

    pub async fn post_signed_beacon_block(block: &SignedBeaconBlock) -> Result<(), Error> {
        unimplemented!("")
    }

    pub async fn post_signed_blinded_beacon_block(
        block: &SignedBlindedBeaconBlock,
    ) -> Result<(), Error> {
        unimplemented!("")
    }

    // v2 endpoint
    pub async fn get_beacon_block(&self, id: BlockId) -> Result<SignedBeaconBlock, Error> {
        let result: Value<SignedBeaconBlock> =
            self.get(&format!("eth/v2/beacon/blocks/{id}")).await?;
        Ok(result.data)
    }

    pub async fn get_beacon_block_root(&self, id: BlockId) -> Result<Root, Error> {
        let result: Value<RootData> = self.get(&format!("eth/v1/beacon/blocks/{id}/root")).await?;
        Ok(result.data.root)
    }

    pub async fn get_attestations_from_beacon_block(
        &self,
        id: BlockId,
    ) -> Result<Vec<Attestation>, Error> {
        let result: Value<Vec<Attestation>> = self
            .get(&format!("eth/v1/beacon/blocks/{id}/attestations"))
            .await?;
        Ok(result.data)
    }

    pub async fn get_attestations_from_pool(
        &self,
        slot: Option<Slot>,
        committee_index: Option<CommitteeIndex>,
    ) -> Result<Vec<Attestation>, Error> {
        let path = "eth/v1/beacon/pool/attestations";
        let target = self.endpoint.join(path)?;
        let mut request = self.http.get(target);
        if let Some(slot) = slot {
            request = request.query(&[("slot", slot)]);
        }
        if let Some(committee_index) = committee_index {
            request = request.query(&[("committe_index", committee_index)]);
        }
        let response = request.send().await?;
        let result: ApiResult<Value<Vec<Attestation>>> = response.json().await?;
        match result {
            ApiResult::Ok(result) => Ok(result.data),
            ApiResult::Err(err) => Err(err.into()),
        }
    }

    pub async fn post_attestations(attestations: &[Attestation]) -> Result<(), Error> {
        unimplemented!("")
    }

    pub async fn get_attester_slashings_from_pool() -> Result<Vec<AttesterSlashing>, Error> {
        unimplemented!("")
    }

    pub async fn post_attester_slashing(attester_slashing: &AttesterSlashing) -> Result<(), Error> {
        unimplemented!("")
    }

    pub async fn get_proposer_slashings_from_pool() -> Result<Vec<ProposerSlashing>, Error> {
        unimplemented!("")
    }

    pub async fn post_proposer_slashing(proposer_slashing: &ProposerSlashing) -> Result<(), Error> {
        unimplemented!("")
    }

    pub async fn post_sync_committee_messages(
        messages: &[SyncCommitteeMessage],
    ) -> Result<(), Error> {
        unimplemented!("")
    }

    pub async fn get_voluntary_exits_from_pool() -> Result<Vec<SignedVoluntaryExit>, Error> {
        unimplemented!("")
    }

    pub async fn post_signed_voluntary_exit(exit: &SignedVoluntaryExit) -> Result<(), Error> {
        unimplemented!("")
    }

    /* config namespace */
    pub async fn get_fork_schedule() -> Result<Vec<Fork>, Error> {
        unimplemented!("")
    }

    pub async fn get_spec() -> Result<HashMap<String, String>, Error> {
        unimplemented!("")
    }

    pub async fn get_deposit_contract_address() -> Result<(ChainId, ExecutionAddress), Error> {
        unimplemented!("")
    }

    /* debug namespace */
    // v2 endpoint
    pub async fn get_state(id: StateId) -> Result<BeaconState, Error> {
        unimplemented!("")
    }

    // v2 endpoint
    pub async fn get_heads() -> Result<Vec<Coordinate>, Error> {
        unimplemented!("")
    }

    /* events namespace */
    // TODO: figure out return type
    pub async fn get_events<T>(topics: &[EventTopic]) -> Result<T, Error> {
        // get back "event: TOPIC, data: T"
        unimplemented!("")
    }

    /* node namespace */
    #[cfg(feature = "peer-id")]
    pub async fn get_node_identity() -> Result<NetworkIdentity, Error> {
        unimplemented!("")
    }

    #[cfg(feature = "peer-id")]
    pub async fn get_node_peers(filters: &[PeerDescriptor]) -> Result<Vec<PeerDescription>, Error> {
        unimplemented!("")
    }

    #[cfg(feature = "peer-id")]
    pub async fn get_peer(peer_id: Multiaddr) -> Result<PeerDescription, Error> {
        unimplemented!("")
    }

    pub async fn get_peer_summary(&self) -> Result<PeerSummary, Error> {
        let result: Value<PeerSummary> = self.get("eth/v1/node/peer_count").await?;
        Ok(result.data)
    }

    pub async fn get_node_version(self) -> Result<String, Error> {
        let result: Value<VersionData> = self.get("eth/v1/node/version").await?;
        Ok(result.data.version)
    }

    pub async fn get_sync_status() -> Result<SyncStatus, Error> {
        unimplemented!("")
    }

    pub async fn get_health(&self) -> Result<HealthStatus, Error> {
        let path = "eth/v1/node/health";
        let target = self.endpoint.join(path)?;
        let request = self.http.get(target);
        let response = request.send().await?;
        let result = match response.status() {
            StatusCode::OK => HealthStatus::Ready,
            StatusCode::PARTIAL_CONTENT => HealthStatus::Syncing,
            StatusCode::SERVICE_UNAVAILABLE => HealthStatus::NotInitialized,
            _ => HealthStatus::Unknown,
        };
        Ok(result)
    }

    /* validator namespace */
    pub async fn get_attester_duties(
        epoch: Epoch,
        indices: &[ValidatorIndex],
    ) -> Result<(Root, Vec<AttestationDuty>), Error> {
        unimplemented!("")
    }

    pub async fn get_proposer_duties(
        &self,
        epoch: Epoch,
    ) -> Result<(Root, Vec<ProposerDuty>), Error> {
        let endpoint = format!("eth/v1/validator/duties/proposer/{epoch}");
        let mut result: Value<Vec<ProposerDuty>> = self.get(&endpoint).await?;
        let dependent_root_value = result.meta.remove("dependent_root").ok_or_else(|| {
            Error::MissingExpectedData("missing `dependent_root` in response".to_string())
        })?;
        let dependent_root: Root = serde_json::from_value(dependent_root_value)?;
        Ok((dependent_root, result.data))
    }

    pub async fn get_sync_committee_duties(
        epoch: Epoch,
        indices: &[ValidatorIndex],
    ) -> Result<(Root, Vec<SyncCommitteeDuty>), Error> {
        unimplemented!("")
    }

    // v2 endpoint
    pub async fn get_block(
        slot: Slot,
        randao_reveal: RandaoReveal,
        graffiti: Bytes32,
    ) -> Result<BeaconBlock, Error> {
        unimplemented!("")
    }

    pub async fn get_blinded_block(
        slot: Slot,
        randao_reveal: RandaoReveal,
        graffiti: Bytes32,
    ) -> Result<BlindedBeaconBlock, Error> {
        unimplemented!("")
    }

    pub async fn get_attestation_data(
        slot: Slot,
        committee_index: CommitteeIndex,
    ) -> Result<AttestationData, Error> {
        unimplemented!("")
    }

    pub async fn get_attestation_aggregate(
        attestation_data_root: Root,
        slot: Slot,
    ) -> Result<Attestation, Error> {
        unimplemented!("")
    }

    pub async fn post_aggregates_with_proofs(
        aggregates_with_proofs: &[SignedAggregateAndProof],
    ) -> Result<(), Error> {
        Ok(())
    }

    pub async fn subscribe_subnets_for_attestation(
        committee_descriptors: &[CommitteeDescriptor],
    ) -> Result<(), Error> {
        Ok(())
    }

    pub async fn subscribe_subnets_for_sync_committee(
        sync_committee_descriptors: &[SyncCommitteeDescriptor],
    ) -> Result<(), Error> {
        Ok(())
    }

    pub async fn get_sync_committee_contribution(
        slot: Slot,
        subcommittee_index: usize,
        beacon_block_root: Root,
    ) -> Result<SyncCommitteeContribution, Error> {
        unimplemented!("")
    }

    pub async fn post_contributions_with_proofs(
        contributions_with_proofs: &[SignedContributionAndProof],
    ) -> Result<(), Error> {
        Ok(())
    }

    pub async fn prepare_proposers(
        &self,
        registrations: &[BeaconProposerRegistration],
    ) -> Result<(), Error> {
        self.post("/eth/v1/validator/prepare_beacon_proposer", registrations)
            .await
    }

    // endpoint for builder registrations
    pub async fn register_validators_with_builders(
        registrations: &[SignedValidatorRegistration],
    ) -> Result<(), Error> {
        Ok(())
    }
}
