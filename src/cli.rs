use crate::{CliArgs, Client, GenesisDetails, StateId};
use ethereum_consensus::primitives::Root;
use url::Url;

pub async fn cli(args: CliArgs) {
    //set up client
    let s = args.node_url;
    let url: Url = Url::parse(&s).unwrap();
    let client = Client::new(url);
    let endpoint = args.endpoint.as_str();

    match endpoint {
        // Match a single value
        "node_version" => println!("{:?}", client.get_node_version().await.unwrap()),
        "genesis" => {
            let out: GenesisDetails = client.get_genesis_details().await.unwrap();
            println!("{:?}", out.genesis_time);
            println!("{:?}", out.genesis_fork_version);
            println!("{:?}", out.genesis_validators_root);
        }
        "root" => {
            let state: StateId;
            if let Some(state_id) = args.state_id {
                if state_id.contains("finalized") |
                    state_id.contains("Finalized") |
                    state_id.contains("FINALIZED")
                {
                    state = StateId::Finalized;
                } else if state_id.contains("justified") |
                    state_id.contains("Justified") |
                    state_id.contains("JUSTIFIED")
                {
                    state = StateId::Justified;
                } else if state_id.contains("genesis") |
                    state_id.contains("Genesis") |
                    state_id.contains("GENESIS")
                {
                    state = StateId::Genesis;
                } else if state_id.contains("0x") {
                    assert_eq!(state_id.as_bytes().len(), 32, "malformed root in request payload");
                    let bytes: [u8; 32] = state_id.as_bytes().try_into().unwrap();
                    state = StateId::Root(Root::from_bytes(bytes));
                } else {
                    let check_numeric = state_id.trim().parse::<u64>();
                    match check_numeric {
                        Ok(_ok) => state = StateId::Slot(check_numeric.unwrap()),
                        Err(_e) => {
                            println!("error in request payload: please check formats");
                            state = StateId::Slot(0)
                        }
                    }
                }
                let out: Root = client.get_state_root(state).await.unwrap();
                println!("{out:?}");
            } else {
                println!("no state-id provided");
            }
        }

        _ => println!("something else"),
    }
}
