mod common;
use http::StatusCode;

#[tokio::test]
async fn test_http_get() {
    let clinet = common::setup();
    let path = "/eth/v1/builder/validators";
    let response = clinet.http_get(path).await.unwrap();
    assert_eq!(&response.url().as_str(), &"http://127.0.0.1:5052/eth/v1/builder/validators");
    dbg!(&response.status());
    assert_eq!(&response.status(), &StatusCode::METHOD_NOT_ALLOWED);
}

//#[tokio::test]
//async fn test_get() {
//    let client = common::setup();
//    let path = "/eth/v1/builder/validatots";
//    let response = client.
//}