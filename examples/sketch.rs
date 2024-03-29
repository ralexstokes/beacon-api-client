use beacon_api_client::{ApiError, ApiResult, Value, VersionedValue};
use ethereum_consensus::{bellatrix::mainnet as bellatrix, capella::mainnet as capella};
use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(tag = "version", content = "data")]
#[serde(rename_all = "lowercase")]
enum BlindedBeaconBlock {
    Bellatrix(bellatrix::BlindedBeaconBlock),
    Capella(capella::BlindedBeaconBlock),
}

fn main() {
    let block = Value { meta: HashMap::new(), data: bellatrix::BlindedBeaconBlock::default() };
    let block_repr = serde_json::to_string(&block).unwrap();
    println!("{block_repr}");

    let version = serde_json::to_value("bellatrix").unwrap();
    let block_with_version = Value {
        meta: HashMap::from_iter([("version".to_string(), version)]),
        data: bellatrix::BlindedBeaconBlock::default(),
    };
    let block_with_version_repr = serde_json::to_string(&block_with_version).unwrap();
    println!("{block_with_version_repr}");

    let block = BlindedBeaconBlock::Bellatrix(Default::default());
    let block_with_version_repr = serde_json::to_string(&block).unwrap();
    println!("{block_with_version_repr}");
    let recovered_block: BlindedBeaconBlock =
        serde_json::from_str(&block_with_version_repr).unwrap();
    println!("{recovered_block:#?}");

    let block = BlindedBeaconBlock::Capella(Default::default());
    let block_with_version_repr = serde_json::to_string(&block).unwrap();
    println!("{block_with_version_repr}");

    let full_success_response = ApiResult::Ok(block.clone());
    let str_repr = serde_json::to_string(&full_success_response).unwrap();
    println!("{str_repr}");

    let recovered_success: ApiResult<VersionedValue<BlindedBeaconBlock>> =
        serde_json::from_str(&str_repr).unwrap();
    println!("{recovered_success:#?}");

    let full_success_response = ApiResult::Ok(VersionedValue {
        payload: block,
        meta: HashMap::from_iter([(
            String::from("finalized_root"),
            serde_json::Value::String("0xdeadbeefcafe".to_string()),
        )]),
    });
    let str_repr = serde_json::to_string(&full_success_response).unwrap();
    println!("{str_repr}");

    let recovered_success: ApiResult<VersionedValue<BlindedBeaconBlock>> =
        serde_json::from_str(&str_repr).unwrap();
    println!("{recovered_success:#?}");

    let full_error_response: ApiResult<Value<bellatrix::BlindedBeaconBlock>> =
        ApiResult::Err(ApiError::try_from((404, "some failure")).unwrap());
    let str_repr = serde_json::to_string(&full_error_response).unwrap();
    println!("{str_repr}");

    let recovered_error: ApiResult<String> = serde_json::from_str(&str_repr).unwrap();
    println!("{recovered_error:#?}");
}
