/// A macro for easily writing zome functions
///
/// # Examples
/// ```
/// # #[macro_use] extern crate hdk;
/// # extern crate holochain_wasm_utils;
/// # extern crate serde;
/// # extern crate serde_json;
/// # #[macro_use] extern crate serde_derive;
/// # use hdk::globals::G_MEM_STACK;
/// # use holochain_wasm_utils::error::RibosomeReturnCode;
/// # fn main() {
/// #[derive(Serialize)]
/// struct CreatePostResponse {
///     author: String,
/// }
///
/// zome_functions! {
///     create_post: |author: String, content: String| {
///
///         // ..snip..
///
///         CreatePostResponse { author: author }
///     }
/// }
/// # }
/// ```
///
#[macro_export]
macro_rules! zome_functions {
    (
        $($func_name:ident : | $($param:ident : $param_type:ty),* | $main_block:expr)+
    ) => (

        $(
            #[no_mangle]
            pub extern "C" fn $func_name(encoded_allocation_of_input: u32) -> u32 {

                // Macro'd InputStruct
                #[derive(Deserialize)]
                struct InputStruct {
                    $($param : $param_type),*
                }

                // Macro'd function body
                fn execute(params: InputStruct) -> impl ::serde::Serialize {
                    let InputStruct { $($param),* } = params;
                    $main_block
                }

                // Actual program
                // Init memory stack
                unsafe {
                    ::hdk::globals::G_MEM_STACK =
                        Some(::holochain_wasm_utils::memory_allocation::SinglePageStack::from_encoded(encoded_allocation_of_input));
                }

                // Deserialize input
                let maybe_input = ::holochain_wasm_utils::memory_serialization::try_deserialize_allocation(encoded_allocation_of_input);
                if let Err(_) = maybe_input {
                    return ::holochain_wasm_utils::error::RibosomeErrorCode::ArgumentDeserializationFailed as u32;
                }
                let input: InputStruct = maybe_input.unwrap();

                // Execute inner function
                let output_obj = execute(input);

                // Serialize output in WASM memory
                unsafe {
                    return ::holochain_wasm_utils::memory_serialization::serialize_into_encoded_allocation(&mut G_MEM_STACK.unwrap(), output_obj) as u32;
                }
            }
        )+
    );
}


#[macro_export]
macro_rules! validations {
    (
        $([ENTRY] $func_name:ident {
            [$package:path]
            | $entry:ident : $entry_type:ty, $ctx:ident : hdk::ValidationData | $main_block:expr
        })+
    ) => (

        $(
            #[no_mangle]
            pub extern "C" fn $func_name(encoded_allocation_of_input: u32) -> u32 {

                // Macro'd InputStruct
                #[derive(Deserialize)]
                struct InputStruct {
                    $entry : $entry_type,
                    $ctx : ::hdk::ValidationData,
                }

                #[derive(Deserialize)]
                struct InputStructGeneric {
                    entry : $entry_type,
                    ctx : ::hdk::ValidationData,
                }

                // Macro'd function body
                fn execute(params: InputStruct) -> Result<(), String> {
                    let InputStruct { $entry, $ctx } = params;
                    $main_block
                }

                // Actual program
                // Init memory stack
                unsafe {
                    ::hdk::globals::G_MEM_STACK =
                        Some(::holochain_wasm_utils::memory_allocation::SinglePageStack::from_encoded(encoded_allocation_of_input));
                }

                // Deserialize input
                let maybe_input = ::holochain_wasm_utils::memory_serialization::try_deserialize_allocation(encoded_allocation_of_input);
                if let Err(err) = maybe_input {
                    hdk::debug(&format!("Error deserializing: {}", err));
                    return ::holochain_wasm_utils::error::RibosomeErrorCode::ArgumentDeserializationFailed as u32;
                }
                let input_generic: InputStructGeneric = maybe_input.unwrap();
                let input = InputStruct {
                    $entry: input_generic.entry,
                    $ctx: input_generic.ctx,
                };


                // Execute inner function
                let output_result = execute(input);

                // Serialize output in WASM memory
                unsafe {
                    return ::holochain_wasm_utils::memory_serialization::serialize_into_encoded_allocation(&mut G_MEM_STACK.unwrap(), output_result) as u32;
                }
            }
        )+
    );
}
