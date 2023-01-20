// use crate::{Client, GenesisDetails, StateId, Config};
// use ethereum_consensus::{primitives::Root};
// use url::Url;
use crate::{Namespace, Method, Config};


// current flow:
    // main calls run_cli
    // run_cli calls parse_args
    // parse_args checks that the provided method is in the provided namespace
    // args is currently a string - sometimes multiple args are provided - they need to be parsed appropriately for specific methods.
    // if namespace & method & args all validate ok, then call func in api_client

pub fn run_cli(args: &Config){
    parse_args(args)
}


pub fn parse_args(args: &Config) {
    let namespace = &args.namespace;
    let mut namespace_method_ok: bool = false;
    
    match check_method_in_ns(&args.method, &namespace){
        true => namespace_method_ok = true,
        false => {},
    }

    //delete me
    println!("namespace and method match up: {:?}", namespace_method_ok);


}

pub fn check_method_in_ns(method: &Method, namespace: &Namespace) -> bool {
    match namespace {
        Namespace::Beacon => {
            match method {
                Method::Genesis=> return true,
                Method::Root=> return true,
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
                _ => return false
            }
        },
        Namespace::Config => {
            match method {
                Method::ForkSchedule => return true,
                Method::Spec=> return true,
                Method::DepositContract => return true,
                _ => return false
            }
        },
        Namespace::Debug => {
            match method {
                Method::State => return true,
                Method::Head=> return true,
                _ => return false
            }
        },
        Namespace::Events => {
            match method {
                Method::Events => return true,
                _ => return false
            }
        },
        Namespace::Node => {
            match method {
                Method::Identity => return true,
                Method::Peer=> return true,
                Method::Peers=> return true,
                Method::PeerCount=>return true,
                Method::Version=> return true,
                Method::Syncing=> return true,
                Method::Health=>return true,
                _ => return false
            }
        },
        _ => return false
    }

}

// pub async fn run_cli(args: &Config) {
//     //set up client
//     let s = &args.node_url;
//     let url: Url = Url::parse(&s).unwrap();
//     let client = Client::new(url);
//     let method = args.method.as_str();

//     match method {
//         // Match on the provided method
//         "node_version" => println!("{:?}", client.get_node_version().await.unwrap()),
//         "genesis" => {
//             let out: GenesisDetails = client.get_genesis_details().await.unwrap();
//             println!("{:?}", out.genesis_time);
//             println!("{:?}", out.genesis_fork_version);
//             println!("{:?}", out.genesis_validators_root);
//         }
//         "root" => {
//             if let Some(state_id) = &args.state_id {
//                 let state = parse_state_id(state_id);
//                 let out = client.get_state_root(state).await.unwrap();
//                 println!("{out:?}");
//             } else {
//                 println!("no state-id provided");
//             }
//         }
//         "fork" => {
//             if let Some(state_id) = &args.state_id {
//                 let state = parse_state_id(state_id);
//                 let out = client.get_fork(state).await.unwrap();
//                 println!("{out:?}");
//             } else {
//                 println!("no state-id provided");
//             }
//         }

//         _ => println!("method missing or invalid - please provide valid method"),
//     }
// }

// pub fn parse_state_id(state_id: &String) -> StateId {
//     let state: StateId;
//     if state_id.contains("finalized") |
//         state_id.contains("Finalized") |
//         state_id.contains("FINALIZED")
//     {
//         state = StateId::Finalized;
//     } else if state_id.contains("justified") |
//         state_id.contains("Justified") |
//         state_id.contains("JUSTIFIED")
//     {
//         state = StateId::Justified;
//     } else if state_id.contains("genesis") |
//         state_id.contains("Genesis") |
//         state_id.contains("GENESIS")
//     {
//         state = StateId::Genesis;
//     } else if state_id.contains("0x") {
//         assert_eq!(state_id.as_bytes().len(), 32, "malformed root in request payload");
//         let bytes: [u8; 32] = state_id.as_bytes().try_into().unwrap();
//         state = StateId::Root(Root::from_bytes(bytes));
//     } else {
//         let check_numeric = state_id.trim().parse::<u64>();
//         match check_numeric {
//             Ok(_ok) => state = StateId::Slot(check_numeric.unwrap()),
//             Err(_e) => {
//                 println!("error in request payload: please check formats");
//                 state = StateId::Slot(0)
//             }
//         }
//     }
//     state
// }

