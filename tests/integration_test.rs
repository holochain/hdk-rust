extern crate holochain_core;
extern crate holochain_core_api;
extern crate holochain_dna;
extern crate test_utils;

use holochain_core_api::*;
use test_utils::*;

#[test]
fn can_use_globals() {
    // Setup the holochain instance
    let wasm =
        create_wasm_from_file("wasm-test/target/wasm32-unknown-unknown/release/test_globals.wasm");
    let dna = create_test_dna_with_wasm("test_zome", "test_cap", wasm);

    let (context, test_logger) = test_context_and_logger("alex");
    let mut hc = Holochain::new(dna.clone(), context).unwrap();

    // Run the holochain instance
    hc.start().expect("couldn't start");

    // Call the exposed wasm function that calls the Commit API function
    let result = hc.call("test_zome", "test_cap", "check_global", r#"{}"#);
    assert!(result.unwrap().is_empty());

    let result = hc.call(
        "test_zome",
        "test_cap",
        "send_tweet",
        r#"{ "author": "bob", "content": "had a boring day" }"#,
    );
    assert_eq!(result.unwrap(), r#"{"ok": true }"#);

    let test_logger = test_logger.lock().unwrap();

    println!("{:?}", *test_logger);

    let tweet_result = hc.call("test_zome", "test_cap", "send_tweet", r#"
        {
            "author": "Terence",
            "content": "Take it easy dude, but take it!"
        }"#).unwrap();

    println!("Tweet result: {:?}", tweet_result);
    assert_eq!(tweet_result, "\"{\\\"ok\\\":true}\"");
}
