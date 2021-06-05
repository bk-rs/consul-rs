extern crate proc_macro;

use syn::parse_macro_input;

mod endpoint;

#[proc_macro]
pub fn endpoint(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as endpoint::Input);
    let output = endpoint::get_output(input);
    output.into()
}
