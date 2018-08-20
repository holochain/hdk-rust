// File holding the public Zome API
// All API Reference documentation should be done here

extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;

extern crate holochain_wasm_utils;

use holochain_wasm_utils::*;

pub mod globals;
pub mod init_globals;

use globals::g_mem_stack;
use globals::APP_GLOBALS;

extern {
    fn hc_debug(encoded_allocation_of_input: i32) -> i32;
}

pub type HashString = String;

//--------------------------------------------------------------------------------------------------
// APP GLOBALS
//--------------------------------------------------------------------------------------------------

lazy_static! {
  pub static ref APP_NAME: String               = APP_GLOBALS.clone().app_name;
  pub static ref APP_DNA_HASH: HashString       = APP_GLOBALS.clone().app_dna_hash;
  pub static ref APP_KEY_HASH: HashString       = APP_GLOBALS.clone().app_key_hash;
  pub static ref APP_AGENT_HASH: HashString     = APP_GLOBALS.clone().app_agent_hash;
  pub static ref APP_AGENT_TOP_HASH: HashString = APP_GLOBALS.clone().app_agent_top_hash;
  pub static ref APP_AGENT_STR: String          = APP_GLOBALS.clone().app_agent_str;
}


//--------------------------------------------------------------------------------------------------
// SYSTEM CONSTS
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


//--------------------------------------------------------------------------------------------------
// API FUNCTIONS
//--------------------------------------------------------------------------------------------------

/// FIXME DOC
/// Returns an application property, which are defined by the app developer.
/// It returns values from the DNA file that you set as properties of your application
/// (e.g. Name, Language, Description, Author, etc.).
pub fn property<S: Into<String>>(_name: S) -> Result<String, ErrorCode> {
    // FIXME
    Err(ErrorCode::FunctionNotImplemented)
}

/// FIXME DOC
pub fn make_hash<S: Into<String>>(_entry_type : S, _entry_data: serde_json::Value) -> Result<HashString, ErrorCode> {
    // FIXME
    Err(ErrorCode::FunctionNotImplemented)
}


/// FIXME DOC
pub fn debug(msg: &str) {
    unsafe {
        let mut mem_stack = g_mem_stack.unwrap();
        let allocation_of_input =  serialize(&mut mem_stack, msg);
        hc_debug(allocation_of_input.encode() as i32);
        mem_stack.deallocate(allocation_of_input)
          .expect("should be able to deallocate input that has been allocated on memory stack");
    }
}


/// FIXME DOC
pub fn call<S: Into<String>>(_zome_name: S, _function_name: S, _arguments: serde_json::Value) -> Result<serde_json::Value, ErrorCode> {
    // FIXME
    Err(ErrorCode::FunctionNotImplemented)
}

/// FIXME DOC
pub fn sign<S: Into<String>>(_doc: S) -> Result<String, ErrorCode> {
    // FIXME
    Err(ErrorCode::FunctionNotImplemented)
}


/// FIXME DOC
pub fn verify_signature<S: Into<String>>(_signature: S, _data: S, _pub_key: S) -> Result<bool, ErrorCode> {
    // FIXME
    Err(ErrorCode::FunctionNotImplemented)
}


/// FIXME DOC
pub fn commit_entry<S: Into<String>>(_entry_type: S, _entry: serde_json::Value) -> Result<HashString, ErrorCode> {
    // FIXME
    Err(ErrorCode::FunctionNotImplemented)
}


/// FIXME DOC
pub fn update_entry<S: Into<String>>(_entry_type: S, _entry: serde_json::Value, _replaces: HashString) -> Result<HashString, ErrorCode> {
    // FIXME
    Err(ErrorCode::FunctionNotImplemented)
}

/// FIXME DOC
pub fn update_agent() -> Result<HashString, ErrorCode> {
    // FIXME
    Err(ErrorCode::FunctionNotImplemented)
}


/// FIXME DOC
/// Commit a Deletion System Entry
pub fn remove_entry<S: Into<String>>(_entry: HashString, _message: S) -> Result<HashString, ErrorCode> {
    // FIXME
    Err(ErrorCode::FunctionNotImplemented)
}


/// FIXME DOC
pub fn get_entry(_entry_hash: HashString) -> Result<serde_json::Value, ErrorCode> {
    // FIXME
    Err(ErrorCode::FunctionNotImplemented)
}


/// FIXME DOC
pub fn link_entries<S: Into<String>>(_base: HashString, _target: HashString, _tag: S) {
    // FIXME
    // Maybe return error if HashStrings are not valid
}


/// FIXME DOC
pub fn get_links<S: Into<String>>(_base: HashString, _tag: S) -> Result<Vec<HashString>, ErrorCode> {
    // FIXME
    Ok(vec![])
}


/// FIXME DOC
pub fn query() -> Result<Vec<String>, ErrorCode> {
    // FIXME
    Err(ErrorCode::FunctionNotImplemented)
}


/// FIXME DOC
pub fn send(_to: HashString, _message: serde_json::Value) -> Result<serde_json::Value, ErrorCode> {
    // FIXME
    Err(ErrorCode::FunctionNotImplemented)
}


/// FIXME DOC
pub fn start_bundle(_timeout: usize, _user_param: serde_json::Value)  {
    // FIXME
}


/// FIXME DOC
pub fn close_bundle(_action: BundleOnClose)  {
    // FIXME
}
