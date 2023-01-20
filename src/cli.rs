use crate::{Client, Config, Method, Namespace};
use url::Url;

pub async fn run_cli(args: &Config) {
    parse_args(args);
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
    let res: bool;
    match namespace {
        Namespace::Beacon => {
            res = matches!(
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
                    Method::Blocks |
                    Method::BlindedBlocks |
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
            res = matches!(method, Method::ForkSchedule | Method::Spec | Method::DepositContract)
        }
        Namespace::Debug => res = matches!(method, Method::State | Method::Head),
        Namespace::Events => res = matches!(method, Method::Events),
        Namespace::Node => {
            res = matches!(
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
    }
    res
}

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
