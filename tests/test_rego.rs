extern crate env_logger;
extern crate opa_rego;
extern crate serde_json;

use opa_rego::rego::Response;

mod common;
use common::read_json_file;

#[test]
fn test_load_example() {
    let _ = env_logger::try_init();
    let s = read_json_file(&"resource/example-response-001.json");
    let res : Response = serde_json::from_str(&s).unwrap();
}

