# Holochain Development Kit for Rust-based Apps

## Overview
`hdk-rust` is a library for Rust-based holochain dApps that makes it easier to develop Holochain zomes. With Holochain, zome functions and validation code are represented as WASM binaries. This library provides bindings for Rust.

## Use
First, [Rust](https://www.rust-lang.org/en-US/install.html) must be installed on your computer.

Being a Rust library, `hdk-rust` can be added as a dependency to any Rust crate. When you generate Rust based Zomes with [holochain-cmd](https://github.com/holochain/holochain-cmd) it will automatically be added as a dependency, and imported into your code.

```rust
[package]
name = "yourappname"
version = "versionnumber"
authors = ["Your Name Here"]

[dependencies]
hdk = { git = "https://github.com/holochain/hdk-rust"}
```

`hdk-rust` includes a macro which should be used for writing your application logic into Zome functions. To use it looks something like this:
```
#[macro_use] extern crate hdk;
extern crate holochain_wasm_utils;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

#[derive(Serialize)]
struct CreatePostResponse {
    author: String,
}

zome_functions! {
    create_post: |author: String, content: String| {

        // ..snip..

        CreatePostResponse { author: author }
    }
}
```

### Specification for App Development
As new features, or changes to the HDK (and the API) are being designed, use cases will be added to an example app and put as changes to a pull request to its [repository](https://github.com/holochain/app-spec-rust). The repository also integrates the feature set available in Holochain's main branch.

### Availability of API Functions
Functions will continue to move from incomplete to complete as this library matures.

The following functions are **complete**:
- debug

The following functions are **incomplete**:
- property
- make_hash
- call
- sign
- verify_signature
- commit_entry
- update_entry
- update_agent
- remove_entry
- get_entry
- link_entries
- get_links
- query
- send
- start_bundle
- close_bundle

## Organization of Code
`global.rs` holds all internal or private globals used by the zome API library, and contains internal global for memory usage, internal global for retrieving all app globals, and invokable functions in the ribosome 

`lib.rs` holds the public zome API where all API reference documentation is (app global variables, system consts, and API functions)

`macro.rs` is a macro for easily writing zome functions

`init_globals.rs` holds the internal/private zome API function that retrieves all the public global values from the ribosome 

`Cargo.toml` manifest files describe dependencies. They introduce two metadata files with bits of projection information, fetch and build dependencies, and invokes Holochain Rust with the correct parameters. 

## Tests

To test...
First, run the following code which builds a wasm file to test with:
```bash
$ cd wasm-test
$ cargo build --debug --target wasm32-unknown-unknown
```
Then, run:
```bash
$ cd ..
$ cargo test
```

### Integration test
A test that sets up and runs a holochain instance, then calls the exposed WASM function that calls the Commit API function. 

### WASM test
Tests WASM utilities.
