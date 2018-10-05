#[macro_use]
extern crate hdk;
extern crate holochain_wasm_utils;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate boolinator;

use boolinator::Boolinator;
use hdk::globals::G_MEM_STACK;
use holochain_wasm_utils::{error::RibosomeReturnCode, memory_serialization::*, memory_allocation::*};
use hdk::RibosomeError;

#[no_mangle]
pub extern "C" fn check_global(encoded_allocation_of_input: u32) -> u32 {
    unsafe {
        G_MEM_STACK = Some(SinglePageStack::from_encoded(encoded_allocation_of_input));
    }

    hdk::debug(&hdk::APP_NAME);
    hdk::debug(&hdk::APP_DNA_HASH);
    hdk::debug(&hdk::APP_AGENT_ID_STR);
    hdk::debug(&hdk::APP_AGENT_KEY_HASH);
    hdk::debug(&hdk::APP_AGENT_INITIAL_HASH);
    hdk::debug(&hdk::APP_AGENT_LATEST_HASH);

    return 0;
}


#[derive(Deserialize, Serialize, Default)]
struct CommitOutputStruct {
    hash: String,
}


#[no_mangle]
pub extern "C" fn check_commit_entry(encoded_allocation_of_input: u32) -> u32 {

    #[derive(Deserialize, Default)]
    struct CommitInputStruct {
        entry_type_name: String,
        entry_content: String,
    }

    unsafe {
        G_MEM_STACK = Some(SinglePageStack::from_encoded(encoded_allocation_of_input));
    }

    // Deserialize and check for an encoded error
    let result = try_deserialize_allocation(encoded_allocation_of_input as u32);
    if let Err(_) = result {
        return RibosomeReturnCode::ArgumentDeserializationFailed as u32;
    }

    let input: CommitInputStruct = result.unwrap();
    let res = hdk::commit_entry(&input.entry_type_name, json!({
        "entry_content": &input.entry_content
    }));

   let res_obj = match res {
        Ok(hash_str) => CommitOutputStruct {
            hash: hash_str
        },
        Err(RibosomeError::RibosomeFailed(err_str)) => {
            unsafe {
                return serialize_into_encoded_allocation(&mut G_MEM_STACK.unwrap(), err_str) as u32;
            }
        },
       Err(_) => unreachable!(),
    };
    unsafe {
        return serialize_into_encoded_allocation(&mut G_MEM_STACK.unwrap(), res_obj) as u32;
    }
}


//
zome_functions! {
    check_commit_entry_macro: |entry_type_name: String, entry_content: String| {
        let res = hdk::commit_entry(&entry_type_name, json!(
            entry_content
        ));
        match res {
            Ok(hash_str) => Ok(CommitOutputStruct { hash: hash_str }),
            Err(RibosomeError::RibosomeFailed(err_str)) => Err(err_str),
            Err(_) => unreachable!(),
        }
    }
}


#[derive(Serialize, Deserialize)]
struct TweetResponse {
    first: String,
    second: String,
}

zome_functions! {
    send_tweet: |author: String, content: String| {

        TweetResponse { first: author,  second: content}
    }
}

#[derive(Serialize, Deserialize)]
struct TestEntryType {
    stuff: String,
}

validations! {
    [ENTRY] validate_testEntryType {
        [hdk::ValidationPackage::Entry]
        |entry: TestEntryType, _ctx: hdk::ValidationData| {
            (entry.stuff != "FAIL")
                .ok_or_else(|| "FAIL content is not allowed".to_string())
        }
    }
}
