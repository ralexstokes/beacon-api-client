use crate::{Client, Config, Method, Namespace};
use url::Url;

pub async fn run_cli(args: &Config) {
    parse_args(args);
    // check_args_for_method(&args);
    let out = call_api(args).await;
    println!("{out:?}");
}

pub fn parse_args(args: &Config) {
    let namespace = &args.namespace;
    match check_method_in_ns(&args.method, namespace) {
        true => {}
        false => panic!("method not in given namespace"),
    }
}

pub fn check_method_in_ns(method: &Method, namespace: &Namespace) -> bool {
    let res: bool = match namespace {
        Namespace::Beacon => {
            matches!(
                method,
                Method::Genesis |
                    Method::Root |
                    Method::Fork |
                    Method::FinalityCheckpoints |
                    Method::Validator |
                    Method::Validators |
                    Method::ValidatorBalances |
                    Method::Committees |
                    Method::SyncCommittees |
                    Method::Header |
                    Method::Headers |
                    Method::Block |
                    Method::PostBlock |
                    Method::PostBlindedBlock |
                    Method::BlockRoot |
                    Method::BlockAttestations |
                    Method::PoolAttestations |
                    Method::PostAttestations |
                    Method::AttesterSlashing |
                    Method::PostAttesterSlashing |
                    Method::ProposerSlashing |
                    Method::PostProposerSlashing |
                    Method::PostSyncCommittees |
                    Method::VoluntaryExits |
                    Method::PostVoluntaryExits
            )
        }
        Namespace::Config => {
            matches!(method, Method::ForkSchedule | Method::Spec | Method::DepositContract)
        }
        Namespace::Debug => matches!(method, Method::State | Method::Head),
        Namespace::Events => matches!(method, Method::Events),
        Namespace::Node => {
            matches!(
                method,
                Method::Identity |
                    Method::Peer |
                    Method::Peers |
                    Method::PeerCount |
                    Method::Version |
                    Method::Syncing |
                    Method::Health
            )
        }
        Namespace::Validator => {
            matches!(
                method,
                Method::GetAttesterDuties |
                    Method::GetProposerDuties |
                    Method::GetSyncCommitteeDuties |
                    Method::GetBlockproposal |
                    Method::GetBlindedBlockProposal |
                    Method::GetAttestationData |
                    Method::GetAttestationAggregate |
                    Method::PostAggregatesWithProofs |
                    Method::SubscribeSubnetsForSyncCommittees |
                    Method::GetSyncCommitteeContribution |
                    Method::PostSyncCommitteeContributionWithProofs |
                    Method::PrepareProposers |
                    Method::RegisterValidatorsWithBuilders,
            )
        }
    };
    res
}

// pub fn check_args_for_method(args: &Config) {
//     // set 1 contains methods requiring no args
//     let no_args = vec![
//         &Method::Genesis,
//         &Method::Headers,
//         &Method::PoolAttestations,
//         &Method::AttesterSlashing,
//         &Method::ProposerSlashing,
//         &Method::VoluntaryExits,
//         &Method::ForkSchedule,
//         &Method::Spec,
//         &Method::DepositContract,
//         &Method::Head,
//         &Method::Identity,
//         &Method::Peers,
//         &Method::PeerCount,
//         &Method::Version,
//         &Method::Syncing,
//         &Method::Health,
//     ];
//     // set 2 contains methods requiring state_id
//     let requires_state_id = vec![
//         &Method::Fork,
//         &Method::State,
//         &Method::FinalityCheckpoints,
//         &Method::Validators,
//         &Method::ValidatorBalances,
//         &Method::Committees,
//         &Method::SyncCommittees,
//         &Method::State,
//     ];
//     //set 3 requires some json payload
//     let requires_payload = vec![
//         &Method::PostBlock,
//         &Method::PostBlindedBlock,
//         &Method::PostAttestations,
//         &Method::PostAttesterSlashing,
//         &Method::PostSyncCommittees,
//         &Method::PostVoluntaryExits,
//     ];
//     let method = &args.method;

//     if no_args.contains(&method) {
//         println!("no required arguments required for the selected method")
//     }
//     if requires_state_id.contains(&method) {
//         assert!(args.state_id.is_some(), "state_id required for this method");
//         println!("checked args against method: is ok");
//     }
//     if method == &Method::Validator | {
//         assert!(
//             args.state_id.is_some() && args.hex_string.is_some(),
//             "state_id AND validator_id required for this method"
//         );
//         println!("checked args against method: is ok");
//     }
//     if method == &Method::Events {
//         assert!(args.list_events.is_some(), "events subscriptions required for this method")
//     }
//     if method == &Method::Peer {
//         assert!(args.id_peers.is_some(), "peer_id is required for this method")
//     }
//     if (method == &Method::Block) |
//         (method == &Method::Header) |
//         (method == &Method::BlockRoot) |
//         (method == &Method::BlockAttestations)
//     {
//         assert!(args.block_id.is_some(), "block_id required for this method");
//         println!("checked args against method: is ok");
//     }
//     if requires_payload.contains(&method) {
//         assert!(args.payload.is_some(), "payload is required for this method");
//         println!("checked args against method: is ok");
//     }
// }

pub async fn call_api(args: &Config) -> String {
    //set up client
    let s = &args.endpoint;
    let url: Url = Url::parse(s).unwrap();
    let client = Client::new(url);
    let method = &args.method;
    match method {
        // Match on the provided method
        Method::Version => client.get_node_version().await.unwrap(),
        _ => panic!("problem in api call"),
    }
}
