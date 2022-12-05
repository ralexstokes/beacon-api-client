mod common;
use crate::common::constants::*;
use beacon_api_client::{CommitteeFilter, PublicKeyOrIndex, StateId, ValidatorStatus};
/*
- get_genesis_details
- get_state_root
- get_fork
- get_finality_checkpoints
- get_validators
- get_validator
- get_balance
- get_committes
- get_sync_committees
- get_beacon_header_at_head
- get_beacon_header_for_parent_root
- get_beacon_header
- post_signed_beacon_block
- post_signed_blinded_beacon_block
*/

#[tokio::test]
async fn test_get_genesis_details() {
    let client = common::setup();
    let genesis_details = client.get_genesis_details().await.unwrap();
    assert_eq!(genesis_details.genesis_time, GENESIS_TIME);
}

#[tokio::test]
async fn test_get_state_root() {
    let client = common::setup();
    let root = client.get_state_root(StateId::Genesis).await.unwrap();
    assert_eq!(root.as_bytes(), GENESIS_STATE_ROOT);
}

#[tokio::test]
async fn test_get_fork() {
    let client = common::setup();
    let data = client.get_fork(StateId::Genesis).await.unwrap();
    assert_eq!(data.current_version.len(), 4);
    let num = data
        .current_version
        .iter()
        .fold(0_u32, |acc, value| (acc << 8) | *value as u32);
    assert_eq!(num, GENESIS_FORK_VERSION);
}
// Seems pointless, it just returns zeros
// #[tokio::test]
// async fn test_get_finality_checkpoints() {
//     let client = common::setup();
//     let root = client.get_finality_checkpoints(StateId::Genesis).await.unwrap();
//     dbg!(root);
//     // assert_eq!(root, GENESIS_STATE_ROOT);
//     assert_eq!(23, 32);
// }

#[tokio::test]
async fn test_get_validators() {
    let client = common::setup();
    let index = 0;
    let vec_validator_summary = client
        .get_validators(
            StateId::Genesis,
            &[PublicKeyOrIndex::Index(index)],
            &[ValidatorStatus::ActiveOngoing],
        )
        .await
        .unwrap();
    assert_eq!(vec_validator_summary[index].index, index);
    assert_eq!(
        vec_validator_summary[index]
            .validator
            .public_key
            .to_string(),
        VALIDATOR_0.to_string()
    );
}

#[tokio::test]
async fn test_get_validator() {
    let client = common::setup();
    let index = 0;
    let validator_summary = client
        .get_validator(StateId::Genesis, PublicKeyOrIndex::Index(index))
        .await
        .unwrap();
    assert_eq!(validator_summary.index, index);
    assert_eq!(
        validator_summary.validator.public_key.to_string(),
        VALIDATOR_0.to_string()
    );
}

#[tokio::test]
async fn test_get_balances() {
    let client = common::setup();
    let index = 0;
    let vec_balances_summary = client
        .get_balances(StateId::Genesis, &[PublicKeyOrIndex::Index(index)])
        .await
        .unwrap();
    assert_eq!(vec_balances_summary[0].index, index);
    assert_eq!(vec_balances_summary[0].balance, 32000000000);
}

#[tokio::test]
async fn test_get_all_committees() {
    let client = common::setup();
    let result = client.get_all_committees(StateId::Genesis).await.unwrap();
    assert_eq!(result.len(), 160);
}

#[tokio::test]
async fn test_get_committees() {
    let client = common::setup();
    let result = client
        .get_committees(StateId::Genesis, CommitteeFilter::default())
        .await
        .unwrap();
    assert_eq!(result.len(), 160);
}
