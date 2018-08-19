
use holochain_wasm_utils::SinglePageStack;
use holochain_wasm_utils::SinglePageAllocation;
use holochain_wasm_utils::try_deserialize_allocation;
use globals;
use globals::g_mem_stack;
use globals::HashString;

extern {
  fn hc_init_globals(encoded_allocation_of_input: i32) -> i32;
}


//#[derive(Serialize, Default)]
//struct InitGlobalsInput {
//  app_name: String,
//}

// WARNING must be in sync with InitGlobalsOutput in core
#[derive(Deserialize, Serialize, Default)]
struct InitGlobalsOutput {
  app_name: String,
  app_dna_hash: String,
  app_key_hash: String,
  app_agent_hash: String,
  app_agent_top_hash: String,
  app_agent_str: String,
}


// Copy input variables in global variables
// So they can be available throughout the Zome WASM
fn set_globals(globals : InitGlobalsOutput) {
  unsafe {
    *globals::APP_NAME.lock().unwrap() = globals.app_name;
    *globals::APP_DNA_HASH.lock().unwrap() = globals.app_dna_hash as HashString;
    *globals::APP_KEY_HASH.lock().unwrap() = globals.app_key_hash as HashString;
    *globals::APP_AGENT_HASH.lock().unwrap() = globals.app_agent_hash as HashString;
    *globals::APP_AGENT_TOP_HASH.lock().unwrap() = globals.app_agent_top_hash as HashString;
    *globals::APP_AGENT_STR.lock().unwrap() = globals.app_agent_str;
  }
}


pub fn set_globals_dummy() {
  unsafe {
    *globals::APP_NAME.lock().unwrap() = "one".to_string();
//    globals::APP_DNA_HASH = "two";
//    globals::APP_KEY_HASH = "three";
//    globals::APP_AGENT_HASH = "four";
//    globals::APP_AGENT_TOP_HASH = "five";
//    globals::APP_AGENT_STR = "six";
  }
}

//-------------------------------------------------------------------------------------------------
// HC INIT GLOBALS - Secret Function Call
//-------------------------------------------------------------------------------------------------

pub fn init_globals() {

  // Call WASMI-able init_globals
  let encoded_allocation_of_result: i32;
  unsafe {
    encoded_allocation_of_result = hc_init_globals(0);
  }
  // Check for ERROR in encoding
  let result = try_deserialize_allocation(encoded_allocation_of_result as u32);
  if result.is_err() {
    // TODO panic
    return;
  }

  // Deserialize complex result stored in memory
  let output : InitGlobalsOutput = result.unwrap();

  // Map output to HDK API
  set_globals(output);

//  Not needed because mem will reset for next call?
//  // Free allocation of init_globals' output
//  let output_allocation = SinglePageAllocation::new(encoded_allocation_of_result as u32);
//  unsafe {
//    let mut mem_stack = SinglePageStack::new_from_encoded(encoded_allocation_of_result as u32);
//    mem_stack.deallocate(output_allocation.unwrap()).expect("deallocate failed");
//  }
}


//-------------------------------------------------------------------------------------------------
//  Generatable Dispatch function
//-------------------------------------------------------------------------------------------------

// /// WASM START function called at startup of the wasm module
// /// Just init the global variables
//#[no_mangle]
//pub extern "C" fn start() -> i32 {
//  let mut mem_stack = SinglePageStack::new_from_encoded(0);
//  hc_init_globals(&mut mem_stack);
//  return 0;
//}


//#[no_mangle]
//pub extern "C" fn start() -> i32 {
//  set_globals_dummy();
//  return 0;
//}
