/// A macro for easily writing zome functions
///
/// # Examples
/// ```
/// # #[macro_use] extern crate hdk;
/// # extern crate holochain_wasm_utils;
/// # extern crate serde;
/// # extern crate serde_json;
/// # #[macro_use] extern crate serde_derive;
///
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
                let mut stack = ::holochain_wasm_utils::SinglePageStack::new_from_encoded(encoded_allocation_of_input);

                #[derive(Deserialize)]
                struct ParamStruct {
                    $($param : $param_type),*
                }

                fn execute(params: ParamStruct) -> impl ::serde::Serialize {
                    let ParamStruct { $($param),* } = params;

                    $main_block
                }

                let input: ParamStruct =  if let Ok(params) = ::holochain_wasm_utils::try_deserialize_allocation(encoded_allocation_of_input) {
                    params
                } else {
                    return ::holochain_wasm_utils::serialize_into_encoded_allocation(&mut stack, r#"{"error": "invalid parameters"}"#) as u32;
                };

                let raw_obj = execute(input);

                let mut stack = ::holochain_wasm_utils::SinglePageStack::new_from_encoded(encoded_allocation_of_input);

                ::holochain_wasm_utils::serialize_into_encoded_allocation(&mut stack, raw_obj) as u32
            }
        )+
    );
}
