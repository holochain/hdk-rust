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
use holochain_wasm_utils::memory_serialization::*;
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

  /// The identity string used when the chain was first initialized.
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
        json!({ "error": self.description() })
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
            RibosomeFailed(error_desc) => error_desc,
            FunctionNotImplemented => "Function not implemented",
            HashNotFound => "Hash not found",
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
pub fn debug(msg: &str) -> Result<(), RibosomeError> {
    let mut mem_stack = unsafe { G_MEM_STACK.unwrap() };
    let maybe_allocation_of_input = serialize(&mut mem_stack, msg);
    if let Err(err_code) = maybe_allocation_of_input {
        return Err(RibosomeError::RibosomeFailed(err_code.to_string()));
    }
    let allocation_of_input = maybe_allocation_of_input.unwrap();
    unsafe {
        hc_debug(allocation_of_input.encode());
    }
    mem_stack
        .deallocate(allocation_of_input)
        .expect("should be able to deallocate input that has been allocated on memory stack");
    Ok(())
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
    entry_type_name: &str,
    entry_content: serde_json::Value,
) -> Result<HashString, RibosomeError> {
    /* TODO: FIXME
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
        entry_type_name: entry_type_name.to_string(),
        entry_content: entry_content.to_string(),
    };
    let maybe_allocation_of_input = serialize(&mut mem_stack, input);
    if let Err(err_code) = maybe_allocation_of_input {
        return Err(RibosomeError::RibosomeFailed(err_code.to_string()));
    }
    let allocation_of_input = maybe_allocation_of_input.unwrap();

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
    
    */
    Err(RibosomeError::FunctionNotImplemented)
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

        // test empty property key parameter
        assert_eq!(r#"HashNotFound"#, property("").err().unwrap().to_string());

        // test unknown property key parameter
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

        // test known property key parameter
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

        // test empty entry type parameter
        // TODO: is this the right error?
        let result = make_hash("", json!("test_data"));
        assert_eq!(Some(RibosomeError::HashNotFound), result.err());
    }

    #[test]
    /// test that make_hash() returns value for valid entry data
    fn test_make_hash_valid() {
        // check whether function implemented
        let result = make_hash("", json!(""));
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // test non-empty entry type parameter w/ various valid forms of entry data
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
    // debug() unit tests
    //

    #[test]
    /// test that debug() returns error for invalid arguments
    fn test_debug() {
        // TODO: fix once function properly spec'd w/ Result returned
        assert!(false);
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

        // test empty zome name parameter
        // TODO: FIXME with proper error value
        assert_eq!(true, call("", "test", json!("test")).is_err());

        // test empty function name parameter
        // TODO: FIXME with proper error value
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

        // test valid zome, function, and argument(s) parameters
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

        // test sign empty data parameter
        assert_eq!(true, sign("").is_ok());

        // test sign non-empty data parameter
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
        // TODO: raise issue re move vs borrow ownership of data to be signed

        // test invalid (i.e., empty string) parameters
        // TODO: FIXME with proper error value
        assert_eq!(true, verify_signature("", "", "").is_err());

        // sign test data
        let data = "test data".to_string();
        let pub_key = get_entry(APP_AGENT_KEY_HASH.to_string()).unwrap();
        let pub_key = pub_key["public_key"].to_string();
        let signed = sign(data.clone()).unwrap();

        // test invalid public key parameter
        // TODO: FIXME with proper error value
        assert_eq!(
            true,
            verify_signature(signed.clone(), data.clone(), "bad key".to_string()).is_err()
        );

        // test invalid signature parameter
        // TODO: FIXME with proper error value
        assert_eq!(
            true,
            verify_signature("bad signature".to_string(), data, pub_key).is_err()
        );
    }

    #[test]
    /// test that verify_signature() returns value for valid arguments
    fn test_verify_signature_valid() {
        // check whether function implemented
        let result = verify_signature("", "", "");
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // sign test data
        let data = "test data".to_string();
        let pub_key = get_entry(APP_AGENT_KEY_HASH.to_string()).unwrap();
        let pub_key = pub_key["public_key"].to_string();
        let signed = sign(data.clone()).unwrap();

        // get agent public key to verify self-signed data
        assert_eq!(true, verify_signature(signed, data, pub_key).is_ok());
    }

    //
    // commit_entry() unit tests
    //

    #[test]
    /// test that commit_entry() returns error for invalid arguments
    fn test_commit_entry_invalid() {
        // check whether function implemented
        let result = commit_entry("", json!(""));
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // invalid (i.e., empty string) arguments
        // TODO: FIXME with proper error value
        assert_eq!(true, commit_entry("", json!("")).is_err());
    }

    #[test]
    /// test that commit_entry() returns ok for valid arguments
    fn test_commit_entry_valid() {
        // check whether function implemented
        let result = commit_entry("", json!(""));
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // invalid (i.e., empty string) arguments
        assert_eq!(true, commit_entry("test", json!("test data")).is_ok());
    }

    //
    // update_entry() unit tests
    //

    #[test]
    /// test that update_entry() returns error for invalid arguments
    fn test_update_entry_invalid() {
        // check whether function implemented
        let result = update_entry("", json!(""), "".to_string());
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // test invalid invalid entry hash
        // TODO: FIXME with proper error value
        assert_eq!(
            true,
            update_entry("test", json!(""), "".to_string()).is_err()
        );

        // test invalid entry type
        // TODO: FIXME with proper error value
        let test_entry = commit_entry("test", json!("test_data")).unwrap();
        assert_eq!(true, update_entry("", json!(""), test_entry).is_err());
    }

    #[test]
    /// test that update_entry() returns ok for valid arguments
    fn test_update_entry_valid() {
        // check whether function implemented
        let result = update_entry("", json!(""), "".to_string());
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // test update on test entry
        let test_entry = commit_entry("test", json!("test data")).unwrap();
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

        // test update agent
        assert_eq!(true, update_agent().is_ok());
    }

    //
    // remove_entry() unit tests
    //

    #[test]
    /// test that remove_entry() returns error for invalid arguments
    fn test_remove_entry_invalid() {
        // check whether function implemented
        let result = remove_entry("".to_string(), "");
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // test invalid (i.e., empty string) parameters
        // TODO: FIXME with proper error value
        assert_eq!(
            true,
            remove_entry("".to_string(), "remove_entry_invalid() test").is_err()
        );
    }

    #[test]
    /// test that remove_entry() returns ok for valid arguments
    fn test_remove_entry_valid() {
        // check whether function implemented
        let result = remove_entry("".to_string(), "");
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // commit test entry
        let test_entry = commit_entry("test", json!("test data")).unwrap();

        // test remove on test entry
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

        // commit test entry
        let test_entry = commit_entry("test", json!("test data")).unwrap();

        // test get test entry
        let result = get_entry(test_entry);
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

        // test null entry hash parameter
        // TODO: FIXME with proper error value
        assert_eq!(true, get_entry("".to_string()).is_err());

        // commit and then remove test entry
        let test_entry = commit_entry("test", json!("test data")).unwrap();
        remove_entry(test_entry.clone(), "test data").unwrap();

        // test get on removed test entry
        let result = get_entry(test_entry);
        assert_eq!(Some(RibosomeError::HashNotFound), result.err());
    }

    //
    // link_entries() unit tests
    //

    #[test]
    /// test that link_entries() returns ok for valid arguments
    fn test_link_entries() {
        // TODO: fix once function properly spec'd w/ Result returned
        assert!(false);
    }

    //
    // get_links() unit tests
    //

    #[test]
    /// test that get_links() returns error for invalid arguments
    fn test_get_links_invalid() {
        // check whether function implemented
        let result = get_links("".to_string(), "");
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // commit & link test entries
        let test_entry_1 = commit_entry("test1", json!("test data 1")).unwrap();
        let test_entry_2 = commit_entry("test2", json!("test data 2")).unwrap();
        link_entries(test_entry_1.clone(), test_entry_2.clone(), "test link");

        // test null entry hash parameter
        // TODO: FIXME with proper error value
        assert_eq!(true, get_links("".to_string(), "test link").is_err());

        // test null link tag parameter
        // TODO: FIXME with proper error value
        assert_eq!(true, get_links(test_entry_1, "").is_err());
    }

    #[test]
    /// test that get_links() returns ok for valid arguments
    fn test_get_links_valid() {
        // check whether function implemented
        let result = get_links("".to_string(), "");
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // commit & link test entries
        let test_entry_1 = commit_entry("test1", json!("test data 1")).unwrap();
        let test_entry_2 = commit_entry("test2", json!("test data 2")).unwrap();
        link_entries(test_entry_1.clone(), test_entry_2.clone(), "test link");

        // test get on test link
        // TODO: verify link end-point entries
        assert_eq!(true, get_links(test_entry_1, "test link").is_ok());
        assert_eq!(true, get_links(test_entry_2, "test link").is_ok());
    }

    //
    // query() unit tests
    //

    #[test]
    /// test query() returns Result
    fn test_query() {
        // check whether function implemented
        let result = query();
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // TODO: fix once function properly spec'd w/ options parameter
        assert!(false);
    }

    //
    // send() unit tests
    //

    #[test]
    /// test send() returns error for invalid parameters
    fn test_send_invalid() {
        // check whether function implemented
        let result = send("".to_string(), json!(""));
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // test null entry hash parameter
        // TODO: FIXME with proper error value
        assert_eq!(true, send("".to_string(), json!("test message")).is_err());
    }

    #[test]
    /// test send() returns ok for valid parameters
    fn test_send_valid() {
        // check whether function implemented
        let result = send("".to_string(), json!(""));
        if let Some(RibosomeError::FunctionNotImplemented) = result.err() {
            assert!(false);
        }

        // test send message to self (i.e., own agent)
        assert_eq!(
            true,
            send(APP_AGENT_KEY_HASH.to_string(), json!("test message")).is_ok()
        );
    }

    //
    // start_bundle() unit tests
    //

    #[test]
    /// test start_bundle() returns error for invalid parameters
    fn test_start_bundle() {
        // TODO: fix once function properly spec'd w/ Result returned
        assert!(false);
    }

    //
    // close_bundle() unit tests
    //

    #[test]
    /// test close_bundle() returns error for invalid parameters
    fn test_close_bundle() {
        // TODO: fix once function properly spec'd w/ Result returned
        assert!(false);
    }

}
