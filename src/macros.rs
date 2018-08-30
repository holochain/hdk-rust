#[macro_export]
macro_rules! zome_functions {
    (
        $($func_name:ident : | $($param:ident : $param_type:ty),* | $main_block:expr)+
    ) => (

        $(
            #[no_mangle]
            pub extern "C" fn $func_name(encoded_allocation_of_input: u32) -> u32 {

                #[derive(Deserialize)]
                struct ParamStruct {
                    $($param : $param_type),*
                }

                fn execute(params: ParamStruct) -> impl ::serde::Serialize {
                    let ParamStruct { $($param),* } = params;

                    $main_block
                }

                let input: ParamStruct = ::holochain_wasm_utils::deserialize_allocation(encoded_allocation_of_input);

                let raw_obj = execute(input);

                let output_data = ::serde_json::to_string(&raw_obj).unwrap();

                let mut stack = ::holochain_wasm_utils::SinglePageStack::new_from_encoded(encoded_allocation_of_input);

                ::holochain_wasm_utils::serialize_into_encoded_allocation(&mut stack, output_data) as u32
            }
        )+
    );
}
