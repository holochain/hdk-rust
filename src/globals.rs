
use holochain_wasm_utils::SinglePageStack;
use init_globals::init_globals;
use init_globals::InitGlobalsOutput;

pub type HashString = String;

//--------------------------------------------------------------------------------------------------
// ZOME APP GLOBALS
//--------------------------------------------------------------------------------------------------

lazy_static! {
  pub static ref APP_GLOBALS: InitGlobalsOutput = init_globals();

  pub static ref APP_NAME: String               = APP_GLOBALS.clone().app_name;
  pub static ref APP_DNA_HASH: HashString       = APP_GLOBALS.clone().app_dna_hash;
  pub static ref APP_KEY_HASH: HashString       = APP_GLOBALS.clone().app_key_hash;
  pub static ref APP_AGENT_HASH: HashString     = APP_GLOBALS.clone().app_agent_hash;
  pub static ref APP_AGENT_TOP_HASH: HashString = APP_GLOBALS.clone().app_agent_top_hash;
  pub static ref APP_AGENT_STR: String          = APP_GLOBALS.clone().app_agent_str;
}

// Internal global for memory usage
pub static mut g_mem_stack : Option<SinglePageStack> = None;


//--------------------------------------------------------------------------------------------------
// ZOME SYSTEM CONSTS
//--------------------------------------------------------------------------------------------------

// HC.Version
const VERSION : u16 = 1;
const VERSION_STR : &'static str = "1";


// HC.HashNotFound
// FIXME keep in sync with HcApiReturnCode?
pub enum ErrorCode {
  FunctionNotImplemented,
  HashNotFound,
}


// HC.Status
// WARNING keep in sync with CRUDStatus
bitflags! {
  pub struct EntryStatus: u8 {
    const LIVE     = 1 << 0;
    const REJECTED = 1 << 1;
    const DELETED  = 1 << 2;
    const MODIFIED = 1 << 3;
  }
}


// HC.GetMask
bitflags! {
  pub struct GetMask: u8 {
    const ENTRY      = 1 << 0;
    const ENTRY_TYPE = 1 << 1;
    const SOURCES    = 1 << 2;
  }
}
// explicit `Default` implementation
impl Default for GetMask {
  fn default() -> GetMask {
    GetMask::ENTRY
  }
}


// HC.LinkAction
pub enum LinkAction {
  ADD,
  DELETE,
}


// HC.PkgReq
pub enum PkgRequest {
  CHAIN,
  CHAIN_OPTION,
  ENTRY_TYPES,
}


// HC.PkgReq.ChainOpt
pub enum ChainOption {
  NONE,
  HEADERS,
  ENTRIES,
  FULL,
}


// HC.Bridge
pub enum BridgeSide {
  FROM,
  TO,
}


// HC.SysEntryType
enum SystemEntryType {
  DNA,
  AGENT,
  KEY,
  HEADERS,
  DELETION,
}


mod bundle_cancel {
  // HC.BundleCancel.Reason
  pub enum Reason {
    USER_CANCEL,
    TIMEOUT,
  }
  // HC.BundleCancel.Response
  pub enum Response {
    OK,
    COMMIT,
  }
}


/// Allowed input for close_bundle()
pub enum BundleOnClose {
  Commit,
  Discard,
}