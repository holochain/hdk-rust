//! File for holding the internal/private zome api function `init_globals`

use holochain_wasm_utils::try_deserialize_allocation;

extern {
  fn hc_init_globals(encoded_allocation_of_input: i32) -> i32;
}

// WARNING must be in sync with InitGlobalsOutput in core
#[derive(Deserialize, Clone)]
pub struct AppGlobals {
  pub app_name: String,
  pub app_dna_hash: String,
  pub app_agent_id_str: String,
  pub app_agent_key_hash: String,
  pub app_agent_initial_hash: String,
  pub app_agent_latest_hash: String,
}

// HC INIT GLOBALS - Secret Api Function
// Retrieve all the public global values from the ribosome
pub fn init_globals() -> AppGlobals {
  // Call WASMI-able init_globals
  let encoded_allocation_of_result : i32;
  unsafe {
    encoded_allocation_of_result = hc_init_globals(0);
  }
  // Deserialize complex result stored in memory
  let result = try_deserialize_allocation(encoded_allocation_of_result as u32);
  if result.is_err() {
    panic!("AppGlobals should deserialize properly");
  }
  result.unwrap()
}
