# Holochain Development Kit for Rust-based Apps

## Overview
Holochain apps are comprised of zomes, and zome functions and validation code are represented as WASM binaries. In order to provide convenient access to the Holochain API functions and callbacks without forcing app developers to deal with pointers and de-/serialization of parameters, we provide HDK libraries that provide bindings for several source languages such as Rust and TypeScript.

This crate is the Rust implementation.

## State
Only empty function stubs!

## Use
global.rs holds all internal or private globals used by the zome API library, and contains internal global for memory usage, internal global for retrieving all app globals, and invokable functions in the ribosome 

lib.rs holds the public zome API where all API reference documentation is (app global variables, system consts, and API functions)

macro.rs is a macro for easily writing zome functions

init_globals.rs holds the internal/private zome API function that retrieves all the public global values from the ribosome 

cargo.toml manifest files describe dependencies. They introduce two metadata files with bits of projection information, fetch and build dependencies, and invokes holochain Rust with the correct parameters.  

### Integration test
A test that sets up and runs a holochain instance, then calls the exposed WASM function that calls the Commit API function. 

### WASM test
Tests WASM utilities. Contains cargo.toml dependencies file.

### App spec 
As new features, or changes to the HDK (and the API) are being designed, they add use cases to an example app and puts those changes as a pull request to its [repository](https://github.com/holochain/app-spec-rust). The repository also integrates the feature set available in Holochain's master branch.

## Contribute
??
