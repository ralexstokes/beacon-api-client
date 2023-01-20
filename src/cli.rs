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
    match namespace {
        Namespace::Beacon => match method {
            Method::Genesis =>  true,
            Method::Root =>  true,
            Method::Fork =>  true,
            Method::FinalityCheckpoints =>  true,
            Method::Validator =>  true,
            Method::Validators =>  true,
            Method::ValidatorBalances =>  true,
            Method::Committees =>  true,
            Method::SyncCommittees =>  true,
            Method::Header =>  true,
            Method::Headers =>  true,
            Method::Block =>  true,
            Method::Blocks =>  true,
            Method::BlindedBlocks =>  true,
            Method::BlockRoot =>  true,
            Method::BlockAttestations =>  true,
            Method::PoolAttestations =>  true,
            Method::PostAttestations =>  true,
            Method::AttesterSlashing =>  true,
            Method::PostAttesterSlashing =>  true,
            Method::ProposerSlashing =>  true,
            Method::PostProposerSlashing =>  true,
            Method::PostSyncCommittees =>  true,
            Method::VoluntaryExits =>  true,
            Method::PostVoluntaryExits =>  true,
            _ =>  false,
        },
        Namespace::Config => match method {
            Method::ForkSchedule =>  true,
            Method::Spec =>  true,
            Method::DepositContract =>  true,
            _ =>  false,
        },
        Namespace::Debug => match method {
            Method::State =>  true,
            Method::Head =>  true,
            _ =>  false,
        },
        Namespace::Events => match method {
            Method::Events =>  true,
            _ =>  false,
        },
        Namespace::Node => match method {
            Method::Identity =>  true,
            Method::Peer =>  true,
            Method::Peers =>  true,
            Method::PeerCount =>  true,
            Method::Version =>  true,
            Method::Syncing =>  true,
            Method::Health =>  true,
            _ =>  false,
        },
    }
}

pub async fn call_api(args: &Config) -> String {
    //set up client
    let s = &args.endpoint;
    let url: Url = Url::parse(s).unwrap();
    let client = Client::new(url);
    let method = &args.method;
    match method {
        // Match on the provided method
        Method::Version =>  client.get_node_version().await.unwrap(),
        _ => panic!("problem in api call"),
    }
}
