use proc_macro2::TokenStream;
use quote::quote;

mod input;
mod output;
mod path_params;
mod query_option_names;

pub use self::input::Input;

pub fn get_output(input: Input) -> TokenStream {
    quote!(#input)
}
