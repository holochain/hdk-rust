#[macro_use]
extern crate hdk;
extern crate holochain_wasm_utils;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use hdk::globals::G_MEM_STACK;
use holochain_wasm_utils::*;

#[no_mangle]
pub extern "C" fn check_global(encoded_allocation_of_input: u32) -> u32 {
    unsafe {
        G_MEM_STACK = Some(SinglePageStack::from_encoded(
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

#[derive(Deserialize, Serialize, Default)]
struct CommitOutputStruct {
  hash: String,
}

zome_functions! {
    check_commit_entry: |entry_type_name: String, entry_content: String| {
        match hdk::commit_entry(&entry_type_name, &entry_content) {
            Ok(hash_str) => CommitOutputStruct {
                hash: hash_str
            },
            Err(e) => CommitOutputStruct {
                hash: "fail".to_string()
            },
        }
    }
    send_tweet: |author: String, content: String| {

        TweetResponse { ok: true }
    }
}
