//! File holding the public Zome API
//! All API Reference documentation should be done here.

pub extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
pub extern crate holochain_wasm_utils;

pub mod globals;
pub mod init_globals;
pub mod macros;

use self::RibosomeError::*;
use globals::*;
use holochain_wasm_utils::{memory_allocation::*, memory_serialization::*};
use std::{error::Error, fmt};

pub type HashString = String;

//--------------------------------------------------------------------------------------------------
// APP GLOBAL VARIABLES
//--------------------------------------------------------------------------------------------------

lazy_static! {
  /// The name of this Holochain taken from its DNA.
  pub static ref APP_NAME: &'static str = &APP_GLOBALS.app_name;

  /// The hash of this Holochain's DNA.
  /// Nodes must run the same DNA to be on the same DHT.
  pub static ref APP_DNA_HASH: &'static HashString = &APP_GLOBALS.app_dna_hash;

  /// The identity string used to initialize this Holochain with `hcadmin init`.
  /// If you used JSON to embed multiple properties (such as FirstName, LastName, Email, etc),
  /// they can be retrieved here as App.Agent.FirstName, etc. (FIXME)
  pub static ref APP_AGENT_ID_STR: &'static str = &APP_GLOBALS.app_agent_id_str;

  /// The hash of your public key.
  /// This is your node address on the DHT.
  /// It can be used for node-to-node messaging with `send` and `receive` functions.
  pub static ref APP_AGENT_KEY_HASH: &'static HashString = &APP_GLOBALS.app_agent_key_hash;

  /// The hash of the first identity entry on your chain (The second entry on your chain).
  /// This is your peer's identity on the DHT.
  pub static ref APP_AGENT_INITIAL_HASH: &'static HashString = &APP_GLOBALS.app_agent_initial_hash;

  /// The hash of the most recent identity entry that has been committed to your chain.
  /// Starts with the same value as APP_AGENT_INITIAL_HASH.
  /// After a call to `update_agent` it will have the value of the hash of the newly committed identity entry.
  pub static ref APP_AGENT_LATEST_HASH: &'static HashString = &APP_GLOBALS.app_agent_latest_hash;
}

//--------------------------------------------------------------------------------------------------
// SYSTEM CONSTS
//--------------------------------------------------------------------------------------------------

// HC.Version
const VERSION: u16 = 1;
const VERSION_STR: &'static str = "1";

// HC.HashNotFound
#[derive(Clone, Debug, PartialEq)]
pub enum RibosomeError {
    RibosomeFailed(String),
    FunctionNotImplemented,
    HashNotFound,
}

impl RibosomeError {
    pub fn new(msg: &str) -> RibosomeError {
        RibosomeError::RibosomeFailed(msg.to_string())
    }

    pub fn to_json(&self) -> serde_json::Value {
        let err_str = match self {
            RibosomeFailed(error_desc) => error_desc,
            FunctionNotImplemented => "Function not implemented",
            HashNotFound => "Hash not found",
        }.to_string();
        json!({ "error": err_str })
    }
}

impl fmt::Display for RibosomeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // @TODO seems weird to use debug for display
        // replacing {:?} with {} gives a stack overflow on to_string() (there's a test for this)
        // what is the right way to do this?
        // @see https://github.com/holochain/holochain-rust/issues/223
        write!(f, "{:?}", self)
    }
}

impl Error for RibosomeError {
    fn description(&self) -> &str {
        match self {
            FunctionNotImplemented => "Function not implemented",
            HashNotFound => "Hash not found",
            RibosomeFailed(error_desc) => error_desc,
        }
    }
}

impl PartialEq<String> for RibosomeError {
    fn eq(&self, failure_msg: &String) -> bool {
        match self {
            RibosomeFailed(msg) => {
                if msg == failure_msg {
                    return true;
                }
                false
            }
            _ => false,
        }
    }
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
  pub struct GetEntryMask: u8 {
    const ENTRY      = 1 << 0;
    const ENTRY_TYPE = 1 << 1;
    const SOURCES    = 1 << 2;
  }
}
// explicit `Default` implementation
impl Default for GetEntryMask {
    fn default() -> GetEntryMask {
        GetEntryMask::ENTRY
    }
}

// HC.LinkAction
pub enum LinkAction {
    Add,
    Delete,
}

// HC.PkgReq
pub enum PkgRequest {
    Chain,
    ChainOption,
    EntryTypes,
}

// HC.PkgReq.ChainOpt
pub enum ChainOption {
    None,
    Headers,
    Entries,
    Full,
}

// HC.Bridge
pub enum BridgeSide {
    From,
    To,
}

// HC.SysEntryType
// WARNING Keep in sync with SystemEntryType in holochain-rust
enum SystemEntryType {
    Dna,
    Agent,
    Key,
    Headers,
    Deletion,
}

mod bundle_cancel {
    // HC.BundleCancel.Reason
    pub enum Reason {
        UserCancel,
        Timeout,
    }
    // HC.BundleCancel.Response
    pub enum Response {
        Ok,
        Commit,
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
pub fn property<S: Into<String>>(_name: S) -> Result<String, RibosomeError> {
    // FIXME
    Err(RibosomeError::FunctionNotImplemented)
}

/// FIXME DOC
pub fn make_hash<S: Into<String>>(
    _entry_type: S,
    _entry_data: serde_json::Value,
) -> Result<HashString, RibosomeError> {
    // FIXME
    Err(RibosomeError::FunctionNotImplemented)
}

/// FIXME DOC
pub fn debug(msg: &str) {
    let mut mem_stack = unsafe { G_MEM_STACK.unwrap() };
    let allocation_of_input = serialize(&mut mem_stack, msg);
    unsafe {
        hc_debug(allocation_of_input.encode());
    }
    mem_stack
        .deallocate(allocation_of_input)
        .expect("should be able to deallocate input that has been allocated on memory stack");
}

/// FIXME DOC
pub fn call<S: Into<String>>(
    _zome_name: S,
    _function_name: S,
    _arguments: serde_json::Value,
) -> Result<serde_json::Value, RibosomeError> {
    // FIXME
    Err(RibosomeError::FunctionNotImplemented)
}

/// FIXME DOC
pub fn sign<S: Into<String>>(_doc: S) -> Result<String, RibosomeError> {
    // FIXME
    Err(RibosomeError::FunctionNotImplemented)
}

/// FIXME DOC
pub fn verify_signature<S: Into<String>>(
    _signature: S,
    _data: S,
    _pub_key: S,
) -> Result<bool, RibosomeError> {
    // FIXME
    Err(RibosomeError::FunctionNotImplemented)
}

/// FIXME DOC
pub fn commit_entry(
    _entry_type_name: &str,
    _entry_content: &str,
) -> Result<HashString, RibosomeError> {
    #[derive(Serialize, Default)]
    struct CommitInputStruct {
        entry_type_name: String,
        entry_content: String,
    }

    #[derive(Deserialize, Serialize, Default)]
    struct CommitOutputStruct {
        hash: String,
    }

    let mut mem_stack: SinglePageStack;
    unsafe {
        mem_stack = G_MEM_STACK.unwrap();
    }

    // Put args in struct and serialize into memory
    let input = CommitInputStruct {
        entry_type_name: _entry_type_name.to_string(),
        entry_content: _entry_content.to_string(),
    };
    let allocation_of_input = serialize(&mut mem_stack, input);

    // Call WASMI-able commit
    let encoded_allocation_of_result: u32;
    unsafe {
        encoded_allocation_of_result = hc_commit_entry(allocation_of_input.encode() as u32);
    }
    // Deserialize complex result stored in memory and check for ERROR in encoding
    let result = try_deserialize_allocation(encoded_allocation_of_result as u32);
    if let Err(err_str) = result {
        return Err(RibosomeError::RibosomeFailed(err_str));
    }
    let output: CommitOutputStruct = result.unwrap();

    // Free result & input allocations and all allocations made inside commit()
    mem_stack
        .deallocate(allocation_of_input)
        .expect("deallocate failed");

    // Return hash
    Ok(output.hash.to_string())
}

/// FIXME DOC
pub fn update_entry<S: Into<String>>(
    _entry_type: S,
    _entry: serde_json::Value,
    _replaces: HashString,
) -> Result<HashString, RibosomeError> {
    // FIXME
    Err(RibosomeError::FunctionNotImplemented)
}

/// FIXME DOC
pub fn update_agent() -> Result<HashString, RibosomeError> {
    // FIXME
    Err(RibosomeError::FunctionNotImplemented)
}

/// FIXME DOC
/// Commit a Deletion System Entry
pub fn remove_entry<S: Into<String>>(
    _entry: HashString,
    _message: S,
) -> Result<HashString, RibosomeError> {
    // FIXME
    Err(RibosomeError::FunctionNotImplemented)
}

/// FIXME DOC
pub fn get_entry(_entry_hash: HashString) -> Result<serde_json::Value, RibosomeError> {
    // FIXME
    Err(RibosomeError::FunctionNotImplemented)
}

/// FIXME DOC
pub fn link_entries<S: Into<String>>(_base: HashString, _target: HashString, _tag: S) {
    // FIXME
    // Maybe return error if HashStrings are not valid
}

/// FIXME DOC
pub fn get_links<S: Into<String>>(
    _base: HashString,
    _tag: S,
) -> Result<Vec<HashString>, RibosomeError> {
    // FIXME
    Ok(vec![])
}

/// FIXME DOC
pub fn query() -> Result<Vec<String>, RibosomeError> {
    // FIXME
    Err(RibosomeError::FunctionNotImplemented)
}

/// FIXME DOC
pub fn send(
    _to: HashString,
    _message: serde_json::Value,
) -> Result<serde_json::Value, RibosomeError> {
    // FIXME
    Err(RibosomeError::FunctionNotImplemented)
}

/// FIXME DOC
pub fn start_bundle(_timeout: usize, _user_param: serde_json::Value) {
    // FIXME
}

/// FIXME DOC
pub fn close_bundle(_action: BundleOnClose) {
    // FIXME
}

//--------------------------------------------------------------------------------------------------
// UNIT TESTS
//--------------------------------------------------------------------------------------------------

/// Unit tests
#[cfg(test)]
mod test {
    use super::*;

    //
    // Ribosome error handling unit tests
    //

    #[test]
    /// test that we can convert an error to a string
    fn test_to_string() {
        let err = RibosomeError::FunctionNotImplemented.to_string();
        assert_eq!(r#"FunctionNotImplemented"#, err)
    }

    #[test]
    /// test that we can get the description for an error
    fn test_description() {
        let err = RibosomeError::FunctionNotImplemented;
        assert_eq!("Function not implemented", err.description())
    }

    //
    // property() unit tests
    //

    #[test]
    /// test that property() returns HashNotFound error for null key
    fn test_property_invalid() {
        // check whether function implemented
        let result = property("");
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // empty property key
        assert_eq!(r#"HashNotFound"#, property("").err().unwrap().to_string());

        // unknown property key
        assert_eq!(
            r#"HashNotFound"#,
            property("unknown").err().unwrap().to_string()
        );
    }

    #[test]
    /// test that property() returns value for known key
    fn test_property_valid() {
        // check whether function implemented
        let result = property("");
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // known property key
        assert_eq!(true, property("Name").is_ok())
    }

    //
    // make_hash() unit tests
    //

    #[test]
    /// test that make_hash() returns value for array entry data
    fn test_make_hash_invalid() {
        // check whether function implemented
        let result = make_hash("", json!(""));
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // empty entry type
        assert_eq!(
            r#"HashNotFound"#, // TODO: is this the right error?
            make_hash("", json!("test_data")).err().unwrap().to_string()
        )
    }

    #[test]
    /// test that make_hash() returns value for valid entry data
    fn test_make_hash_valid() {
        // check whether function implemented
        let result = make_hash("", json!(""));
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // non-empty entry type w/ various valid forms of entry data
        assert_eq!(true, make_hash("test", json!("")).is_ok());
        assert_eq!(true, make_hash("test", json!("test")).is_ok());
        assert_eq!(true, make_hash("test", json!(1)).is_ok());
        assert_eq!(true, make_hash("test", json!([1, 2, 3])).is_ok());
        assert_eq!(true, make_hash("test", serde_json::Value::Null).is_ok());
        assert_eq!(
            true,
            make_hash("test", serde_json::Value::Bool(true)).is_ok()
        );
        assert_eq!(
            true,
            make_hash("test", json!({"a": [1, 2, 3], "b": true})).is_ok()
        )
    }

    //
    // call() unit tests
    //

    #[test]
    /// test that call() returns error for invalid arguments
    fn test_call_invalid() {
        // check whether function implemented
        let result = call("", "", json!(""));
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // empty zome name
        assert_eq!(true, call("", "test", json!("test")).is_err());

        // empty function name
        assert_eq!(true, call("test", "", json!("test")).is_err());
    }

    #[test]
    /// test that call() returns value for valid arguments
    fn test_call_valid() {
        // check whether function implemented
        let result = call("", "", json!(""));
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // valid zome, function, and argument(s)
        assert_eq!(true, call("test", "test", json!("")).is_ok());
    }

    //
    // sign() unit tests
    //

    #[test]
    /// test that sign() returns value for valid arguments
    fn test_sign() {
        // check whether function implemented
        let result = sign("");
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        assert_eq!(true, sign("").is_ok());
        assert_eq!(true, sign("test data").is_ok());
    }

    //
    // verify_signature() unit tests
    //

    #[test]
    /// test that verify_signature() returns error for invalid arguments
    fn test_verify_signature_invalid() {
        // check whether function implemented
        let result = verify_signature("", "", "");
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // invalid (i.e., empty string) arguments
        assert_eq!(true, verify_signature("", "", "").is_err());

        // invalid public key used to verify signature on self-signed test data
        // TODO: raise issue re move vs borrow ownership of data to be signed
        let data = "test data".to_string();
        let signed_data = sign(data.clone());
        if let Ok(sig) = signed_data {
            // test signature invalid per bad signing key
            let bad_key = "".to_string();
            assert_eq!(true, verify_signature(sig, data, bad_key).is_err());
        }
    }

    #[test]
    /// test that verify_signature() returns value for valid arguments
    fn test_verify_signature_valid() {
        // check whether function implemented
        let result = verify_signature("", "", "");
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // get agent public key to verify self-signed data
        let key_entry = get_entry(APP_AGENT_KEY_HASH.to_string());
        if let Ok(entry_value) = key_entry {
            let pub_key = entry_value["public_key"].to_string();
            let data = "test data".to_string();
            let signed_data = sign(data.clone());
            if let Ok(sig) = signed_data {
                assert_eq!(true, verify_signature(sig, data, pub_key).is_ok());
            }
        }
    }

    //
    // commit_entry() unit tests
    //

    #[test]
    /// test that commit_entry() returns error for invalid arguments
    ///
    fn test_commit_entry_invalid() {
        // check whether function implemented
        let result = commit_entry("", "");
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // invalid (i.e., empty string) arguments
        assert_eq!(true, commit_entry("", "").is_err());
    }

    #[test]
    /// test that commit_entry() returns ok for valid arguments
    ///
    fn test_commit_entry_valid() {
        // check whether function implemented
        let result = commit_entry("", "");
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // invalid (i.e., empty string) arguments
        assert_eq!(true, commit_entry("test", "test data").is_ok());
    }

    //
    // update_entry() unit tests
    //

    #[test]
    /// test that update_entry() returns error for invalid arguments
    ///
    fn test_update_entry_invalid() {
        // check whether function implemented
        let result = update_entry("", json!(""), "".to_string());
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // invalid entry
        let bad_entry = "".to_string();
        assert_eq!(true, update_entry("test", json!(""), bad_entry).is_err());

        // invalid entry type
        let test_entry = commit_entry("test", "test_data").unwrap();
        assert_eq!(true, update_entry("", json!(""), test_entry).is_err());
    }

    #[test]
    /// test that entry() returns ok for valid arguments
    ///
    fn test_update_entry_valid() {
        // check whether function implemented
        let result = update_entry("", json!(""), "".to_string());
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // invalid (i.e., empty string) arguments
        let test_entry = commit_entry("test", "test data").unwrap();
        assert_eq!(
            true,
            update_entry("test", json!("test data"), test_entry).is_ok()
        );
    }

    //
    // update_agent() unit tests
    //

    #[test]
    /// test that update_agent() returns ok
    fn test_update_agent() {
        // check whether function implemented
        let result = update_agent();
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // valid invocation
        assert_eq!(true, update_agent().is_ok());
    }

    //
    // commit_entry() unit tests
    //

    #[test]
    /// test that remove_entry() returns error for invalid arguments
    fn test_remove_entry_invalid() {
        // check whether function implemented
        let result = remove_entry("".to_string(), "");
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // invalid (i.e., empty string) arguments
        assert_eq!(
            true,
            remove_entry("".to_string(), "remove_entry_invalid() test").is_err()
        );
    }

    #[test]
    /// test that remove_entry() returns ok for valid arguments
    ///
    fn test_remove_entry_valid() {
        // check whether function implemented
        let result = remove_entry("".to_string(), "");
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // invalid (i.e., empty string) arguments
        let test_entry = commit_entry("test", "test data").unwrap();
        assert_eq!(
            true,
            remove_entry(test_entry, "remove_entry_valid() test").is_ok()
        );
    }

    //
    // get_entry() unit tests
    //

    #[test]
    /// test that get_entry() returns ok for valid arguments
    fn test_get_entry_valid() {
        // check whether function implemented
        let result = get_entry("".to_string());
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // get newly-committed test entry
        let test_entry = commit_entry("test", "test data").unwrap();
        let result = get_entry(test_entry);
        assert_eq!(true, result.is_ok());
        assert_eq!(Some(json!("test data")), result.ok());
    }

    #[test]
    /// test that get_entry() returns error for valid arguments
    fn test_get_entry_invalid() {
        // check whether function implemented
        let result = get_entry("".to_string());
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // null entry hash
        assert_eq!(true, get_entry("".to_string()).is_err());

        // get removed test entry
        let test_entry = commit_entry("test", "test data").unwrap();
        remove_entry(test_entry.clone(), "test data").unwrap();
        let result = get_entry(test_entry);
        assert_eq!(true, result.is_err());
        if let Some(RibosomeError::HashNotFound) = result.err() {
            assert!(true);
        }
    }

    //
    // link_entries() unit tests
    //

    #[test]
    /// test that link_entries() returns ok for valid arguments
    fn test_link_entries() {
        // TODO: fix once function properly spec'd w/ Result returned
    }

    //
    // get_links() unit tests
    //

    #[test]
    /// test that link_entries() returns error for invalid arguments
    fn test_get_links_invalid() {
        // check whether function implemented
        let result = get_links("".to_string(), "");
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // created link between newly-created test entries
        let test_entry_1 = commit_entry("test1", "test data 1").unwrap();
        let test_entry_2 = commit_entry("test2", "test data 2").unwrap();
        link_entries(test_entry_1.clone(), test_entry_2.clone(), "test link");

        // null entry
        assert_eq!(true, get_links("".to_string(), "test link").is_err());

        // null link tag
        assert_eq!(true, get_links(test_entry_1, "").is_err());
    }

    #[test]
    /// test that link_entries() returns ok for valid arguments
    fn test_get_links_valid() {
        // check whether function implemented
        let result = get_links("".to_string(), "");
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // created link between newly-created test entries
        let test_entry_1 = commit_entry("test1", "test data 1").unwrap();
        let test_entry_2 = commit_entry("test2", "test data 2").unwrap();
        link_entries(test_entry_1.clone(), test_entry_2.clone(), "test link");

        // get test link
        assert_eq!(true, get_links(test_entry_1, "test link").is_ok());
    }
}
