use proc_macro2::TokenStream;
use quote::quote;

mod input;
mod path_params;
mod query_option_names;

pub use self::input::Input;

pub fn get_output(input: Input) -> TokenStream {
    if input.path.is_empty() {
        let err = "path missing or empty";
        return quote!(compile_error!(#err));
    }

    unimplemented!()
}
