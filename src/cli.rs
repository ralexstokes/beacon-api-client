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
            if args.payload.contains("finalized") |
                args.payload.contains("Finalized") |
                args.payload.contains("FINALIZED")
            {
                state = StateId::Finalized;
            } else if args.payload.contains("justified") |
                args.payload.contains("Justified") |
                args.payload.contains("JUSTIFIED")
            {
                state = StateId::Justified;
            } else if args.payload.contains("genesis") |
                args.payload.contains("Genesis") |
                args.payload.contains("GENESIS")
            {
                state = StateId::Genesis;
            } else if args.payload.contains("0x") {
                assert_eq!(args.payload.as_bytes().len(), 32, "malformed root in request payload");
                let bytes: [u8; 32] = args.payload.as_bytes().try_into().unwrap();
                state = StateId::Root(Root::from_bytes(bytes));
            } else {
                let check_numeric = args.payload.trim().parse::<u64>();
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
        }

        _ => println!("something else"),
    }
}
