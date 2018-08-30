#[macro_use]
extern crate hdk;
extern crate holochain_wasm_utils;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use hdk::globals::g_mem_stack;
use holochain_wasm_utils::*;

#[no_mangle]
pub extern "C" fn check_global_dispatch(encoded_allocation_of_input: u32) -> u32 {
    unsafe {
        g_mem_stack = Some(SinglePageStack::new_from_encoded(
            encoded_allocation_of_input,
        ));
    }
    hdk::debug(&hdk::APP_NAME);
    hdk::debug(&hdk::APP_DNA_HASH);
    hdk::debug(&hdk::APP_AGENT_ID_STR);
    hdk::debug(&hdk::APP_AGENT_KEY_HASH);
    hdk::debug(&hdk::APP_AGENT_INITIAL_HASH);
    hdk::debug(&hdk::APP_AGENT_LATEST_HASH);

    return 0;
}

#[derive(Serialize)]
struct TweetResponse {
    ok: bool,
}

zome_functions! {
    send_tweet: |author: String, content: String| {
        println!("Author: {}, content: {}", author, content);

        TweetResponse { ok: true }
    }
}
