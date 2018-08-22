
extern crate hdk;
extern crate holochain_wasm_utils;

use holochain_wasm_utils::*;
use hdk::globals::g_mem_stack;

#[no_mangle]
pub extern "C" fn check_global_dispatch(encoded_allocation_of_input : i32) -> i32 {
  unsafe {
    g_mem_stack = Some(SinglePageStack::new_from_encoded(encoded_allocation_of_input as u32));

    hdk::debug(&hdk::APP_NAME);
    hdk::debug(&hdk::APP_DNA_HASH);
    hdk::debug(&hdk::APP_AGENT_ID_STR);
    hdk::debug(&hdk::APP_AGENT_KEY_HASH);
    hdk::debug(&hdk::APP_AGENT_INITIAL_HASH);
    hdk::debug(&hdk::APP_AGENT_LATEST_HASH);
  }
  return 0;
}
