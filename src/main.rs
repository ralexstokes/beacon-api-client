use beacon_api_client::{
    BeaconMethod, CliConfig, ConfigMethod, DebugMethod, EventsMethod,
    Namespace::{Beacon, Config, Debug, Events},
    StateIdArg,
};
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = CliConfig::parse();

    println!("{:?}", args);
    match args.command {
        Beacon(BeaconMethod::Root(ref state_id)) => state_id.execute(&args).await,
        Config(ConfigMethod::ForkSchedule) => println!("not ready yet"),
        Debug(DebugMethod::State(state_id)) => println!("not ready yet"),
        Events(EventsMethod::Events) => println!("not ready yet"),
        _ => println!("coming later"),
    }
}
