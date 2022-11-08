use beacon_api_client::Client;
use url::Url;

pub fn setup() -> Client {
    let s = "http://127.0.0.1:5052/";
    let url: Url = Url::parse(s).unwrap();
    let client = Client::new(url);
    client
}
