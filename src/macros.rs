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
