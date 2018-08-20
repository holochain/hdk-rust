//! File for holding all internal/private globals used by the zome api library

use holochain_wasm_utils::SinglePageStack;
use init_globals::init_globals;
use init_globals::AppGlobals;

// Internal global for memory usage
pub static mut g_mem_stack : Option<SinglePageStack> = None;

// Internal global for retrieving all app globals
lazy_static! {
  pub static ref APP_GLOBALS: AppGlobals = init_globals();
}
