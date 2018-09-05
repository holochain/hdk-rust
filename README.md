# Holochain Development Kit for Rust-based Apps

## Overview
`holochain-hdk-rust` is a library for Rust-based holochain dApps that makes it easier to develop holochain zomes. With holochain, zome functions and validation code are represented as WASM binaries. This library provides bindings for Rust. 

## Use
First, [Rust](https://www.rust-lang.org/en-US/install.html) must be installed on your computer. 

The development kit functions like a Rust packet manager, a crate. `holochain-hdk-rust` can be added as a dependency for any new dApp project. 

```rust
[package]
name = "yourappname"
version = "versionnumber"
authors = ["Your Name Here"]

[dependencies]
hdk = { git = "https://github.com/holochain/hdk-rust"}
```

For example, macro.rs provides a boilerplate for writing zome functions.

TODO pending zome functions example doc

### App spec 
As new features, or changes to the HDK (and the API) are being designed, they add use cases to an example app and puts those changes as a pull request to its [repository](https://github.com/holochain/app-spec-rust). The repository also integrates the feature set available in Holochain's master branch.

### Availability of API Functions
Functions will continue to move from incomplete to complete as this library matures.

The following functions are <b>complete</b>:

debug

The following functions are <b>incomplete</b>:

property

make_hash

call

sign

verify_signature

commit_entry

update_entry

update_agent

remove_entry

get_entry

link_entries

get_links

query

send

start_bundle

close_bundle

## Organization of Code
global.rs holds all internal or private globals used by the zome API library, and contains internal global for memory usage, internal global for retrieving all app globals, and invokable functions in the ribosome 

lib.rs holds the public zome API where all API reference documentation is (app global variables, system consts, and API functions)

macro.rs is a macro for easily writing zome functions

init_globals.rs holds the internal/private zome API function that retrieves all the public global values from the ribosome 

cargo.toml manifest files describe dependencies. They introduce two metadata files with bits of projection information, fetch and build dependencies, and invokes holochain Rust with the correct parameters. 

## Tests

### Integration test
A test that sets up and runs a holochain instance, then calls the exposed WASM function that calls the Commit API function. 

### WASM test
Tests WASM utilities. Contains cargo.toml dependencies file.
