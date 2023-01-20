use crate::{Config, Method, Namespace, Client};
use url::{Url};

pub async fn run_cli(args: &Config) {
    parse_args(&args);
    let out = call_api(&args).await;
    println!("{:?}", out);
}

pub fn parse_args(args: &Config) {
    let namespace = &args.namespace;
    match check_method_in_ns(&args.method, &namespace) {
        true => {},
        false => panic!("method not in given namespace"),
    }
}

pub fn check_method_in_ns(method: &Method, namespace: &Namespace) -> bool {
    match namespace {
        Namespace::Beacon => match method {
            Method::Genesis => return true,
            Method::Root => return true,
            Method::Fork => return true,
            Method::FinalityCheckpoints => return true,
            Method::Validator => return true,
            Method::Validators => return true,
            Method::ValidatorBalances => return true,
            Method::Committees => return true,
            Method::SyncCommittees => return true,
            Method::Header => return true,
            Method::Headers => return true,
            Method::Block => return true,
            Method::Blocks => return true,
            Method::BlindedBlocks => return true,
            Method::BlockRoot => return true,
            Method::BlockAttestations => return true,
            Method::PoolAttestations => return true,
            Method::PostAttestations => return true,
            Method::AttesterSlashing => return true,
            Method::PostAttesterSlashing => return true,
            Method::ProposerSlashing => return true,
            Method::PostProposerSlashing => return true,
            Method::PostSyncCommittees => return true,
            Method::VoluntaryExits => return true,
            Method::PostVoluntaryExits => return true,
            _ => return false,
        },
        Namespace::Config => match method {
            Method::ForkSchedule => return true,
            Method::Spec => return true,
            Method::DepositContract => return true,
            _ => return false,
        },
        Namespace::Debug => match method {
            Method::State => return true,
            Method::Head => return true,
            _ => return false,
        },
        Namespace::Events => match method {
            Method::Events => return true,
            _ => return false,
        },
        Namespace::Node => match method {
            Method::Identity => return true,
            Method::Peer => return true,
            Method::Peers => return true,
            Method::PeerCount => return true,
            Method::Version => return true,
            Method::Syncing => return true,
            Method::Health => return true,
            _ => return false,
        }
    }
}

pub async fn call_api(args: &Config)->String{
    //set up client
    let s = &args.endpoint;
    let url: Url = Url::parse(&s).unwrap();
    let client = Client::new(url);
    let method = &args.method;
    match method {
        // Match on the provided method
        Method::Version => return client.get_node_version().await.unwrap(),
        _ => panic!("problem in api call"),
    }
}
