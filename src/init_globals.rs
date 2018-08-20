
use holochain_wasm_utils::try_deserialize_allocation;

extern {
  fn hc_init_globals(encoded_allocation_of_input: i32) -> i32;
}

// WARNING must be in sync with InitGlobalsOutput in core
#[derive(Deserialize, Clone)]
pub struct InitGlobalsOutput {
  pub app_name: String,
  pub app_dna_hash: String,
  pub app_key_hash: String,
  pub app_agent_hash: String,
  pub app_agent_top_hash: String,
  pub app_agent_str: String,
}

// HC INIT GLOBALS - Secret Api Function
pub fn init_globals() -> InitGlobalsOutput {
  // Call WASMI-able init_globals
  let encoded_allocation_of_result : i32;
  unsafe {
    encoded_allocation_of_result = hc_init_globals(0);
  }
  // Deserialize complex result stored in memory
  let result = try_deserialize_allocation(encoded_allocation_of_result as u32);
  if result.is_err() {
    panic!("InitGlobalsOutput should deserialize properly");
  }
  result.unwrap()
}