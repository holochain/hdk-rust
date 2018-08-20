
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
    hdk::debug(&hdk::APP_KEY_HASH);
    hdk::debug(&hdk::APP_AGENT_HASH);
    hdk::debug(&hdk::APP_AGENT_TOP_HASH);
    hdk::debug(&hdk::APP_AGENT_STR);
  }
  return 0;
}