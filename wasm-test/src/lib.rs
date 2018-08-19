#![no_main]
#![no_std]
#![feature(lang_items)]
#![feature(start)]
// https://github.com/rust-lang/rust/issues/29633

extern crate hdk;

extern crate holochain_wasm_utils;



use holochain_wasm_utils::*;

use hdk::globals;
use hdk::init_globals::*;
use hdk::globals::g_mem_stack;

#[no_mangle]
pub extern "C" fn check_global_dispatch(encoded_allocation_of_input : i32) -> i32 {
  unsafe {
    g_mem_stack = Some(SinglePageStack::new_from_encoded(encoded_allocation_of_input as u32));

    hdk::print_debug(&globals::APP_NAME.lock().unwrap());
    hdk::print_debug(&globals::APP_DNA_HASH.lock().unwrap());
    hdk::print_debug(&globals::APP_KEY_HASH.lock().unwrap());
    hdk::print_debug(&globals::APP_AGENT_HASH.lock().unwrap());
    hdk::print_debug(&globals::APP_AGENT_TOP_HASH.lock().unwrap());
    hdk::print_debug(&globals::APP_AGENT_STR.lock().unwrap());
  }
  return 0;
}


#[start]
#[no_mangle]
pub extern "C" fn main() -> i32 {
  set_globals_dummy();
  return 0;
}

//#[no_mangle]
//#[lang = "start"]
//pub extern "C" fn main() -> i32 {
//  set_globals_dummy();
//  return 0;
//}


#[no_mangle]
pub extern "C" fn manual_start() -> i32 {
  //init_globals();
  return 0;
}
