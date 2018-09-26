extern crate holochain_core;
extern crate holochain_core_api;
extern crate holochain_dna;
extern crate test_utils;

use holochain_core_api::*;
use std::sync::{Arc, Mutex};
use test_utils::*;

fn start_holochain_instance() -> (Holochain, Arc<Mutex<TestLogger>>) {
    // Setup the holochain instance
    let wasm =
        create_wasm_from_file("wasm-test/target/wasm32-unknown-unknown/release/test_globals.wasm");
    let dna = create_test_dna_with_wasm("test_zome", "test_cap", wasm);

    let (context, test_logger) = test_context_and_logger("alex");
    let mut hc = Holochain::new(dna.clone(), context).unwrap();

    // Run the holochain instance
    hc.start().expect("couldn't start");
    (hc, test_logger)
}

#[test]
fn can_use_globals() {
    let (mut hc, _) = start_holochain_instance();
    // Call the exposed wasm function that calls the debug API function for printing all GLOBALS
    let result = hc.call("test_zome", "test_cap", "check_global", r#"{}"#);
    println!("{:?}", result);
    assert!(result.unwrap().is_empty());
}

#[test]
fn can_commit_entry() {
    let (mut hc, _) = start_holochain_instance();
    // Call the exposed wasm function that calls the Commit API function
    let result = hc.call(
        "test_zome",
        "test_cap",
        "check_commit_entry",
        r#"{ "entry_type_name": "typename1", "entry_content": "some content" }"#,
    );
    println!("\t result = {:?}", result);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "{\"hash\":\"QmR6vwie4jZiLeUiMZwJjTRMzS55ZMbMrXUCXcwRb3kTt9\"}"
    );
}

#[test]
fn can_commit_entry_macro() {
    let (mut hc, _) = start_holochain_instance();
    // Call the exposed wasm function that calls the Commit API function
    let result = hc.call(
        "test_zome",
        "test_cap",
        "check_commit_entry_macro",
        r#"{ "entry_type_name": "typename1", "entry_content": "some content" }"#,
    );
    println!("\t result = {:?}", result);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "{\"hash\":\"QmR6vwie4jZiLeUiMZwJjTRMzS55ZMbMrXUCXcwRb3kTt9\"}"
    );
}

#[test]
fn can_round_trip() {
    let (mut hc, test_logger) = start_holochain_instance();
    let result = hc.call(
        "test_zome",
        "test_cap",
        "send_tweet",
        r#"{ "author": "bob", "content": "had a boring day" }"#,
    );
    assert_eq!(
        result.unwrap(),
        "{\"first\":\"bob\",\"second\":\"had a boring day\"}"
    );

    let test_logger = test_logger.lock().unwrap();

    println!("{:?}", *test_logger);
}
