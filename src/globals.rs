
use holochain_wasm_utils::SinglePageStack;
use std::sync::Mutex;

pub type HashString = String;

//--------------------------------------------------------------------------------------------------
// ZOME APP GLOBALS
//--------------------------------------------------------------------------------------------------

// From https://github.com/rust-lang-nursery/lazy-static.rs/issues/56
lazy_static! {
  pub static ref APP_NAME : Mutex<String> = Mutex::new("1".to_string());
  pub static ref APP_DNA_HASH : Mutex<HashString> = Mutex::new("2".to_string());
  pub static ref APP_KEY_HASH : Mutex<HashString> = Mutex::new("3".to_string());
  pub static ref APP_AGENT_HASH : Mutex<HashString> = Mutex::new("4".to_string());
  pub static ref APP_AGENT_TOP_HASH : Mutex<HashString> = Mutex::new("5".to_string());
  pub static ref APP_AGENT_STR : Mutex<String> = Mutex::new("6".to_string());
}

//pub static mut APP_NAME : &'static str = "un";
//pub static mut APP_DNA_HASH : &'static str = "deux";
//pub static mut APP_KEY_HASH : &'static str = "trois";
//pub static mut APP_AGENT_HASH : &'static str = "quatre";
//pub static mut APP_AGENT_TOP_HASH : &'static str = "cinq";
//pub static mut APP_AGENT_STR : &'static str = "six";

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