use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt as _};
use syn::Expr;

use super::{path_params::PathParams, query_option_names::QueryOptionNames};

struct ParameterFunctions {
    query_option_names: Option<QueryOptionNames>,
}
impl ToTokens for ParameterFunctions {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(query_option_names) = &self.query_option_names {
            for query_option_name in query_option_names.0.iter() {
                let (field_name, field_type) = query_option_name.field();

                let fn_name = format_ident!("set_parameter_{}", field_name);

                let token = quote! {
                    pub fn #fn_name(&mut self, val: #field_type) -> &mut Self {
                        self.query_options.#field_name = val;
                        self
                    }
                };

                tokens.append_all(token);
            }
        }
    }
}

struct MethodFunction {
    method: Option<Expr>,
}
impl ToTokens for MethodFunction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let token = if let Some(method) = &self.method {
            quote! {
                fn method(&self) -> ::http::Method {
                    #method
                }
            }
        } else {
            quote! {
                fn method(&self) -> ::http::Method {
                    ::http::Method::GET
                }
            }
        };

        tokens.append_all(token);
    }
}

struct PathFunction {
    path: String,
    path_params: Option<PathParams>,
}
impl ToTokens for PathFunction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path = &self.path;

        let token = if let Some(path_params) = &self.path_params {
            let path_params: Vec<_> = path_params
                .0
                .iter()
                .map(|(name, ty)| {
                    quote! {
                        #name = #ty,
                    }
                })
                .collect();
            quote! {
                fn path(&self) -> String {
                    format!(#path, #(#path_params)*)
                }
            }
        } else {
            quote! {
                fn path(&self) -> String {
                    #path
                }
            }
        };

        tokens.append_all(token);
    }
}
