use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt as _};
use syn::{Expr, Type};

use super::{input::Input, path_params::PathParams, query_option_names::QueryOptionNames};

impl ToTokens for Input {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let def_struct = DefStruct {
            name: self.name.to_owned(),
            path_params: self.path_params.to_owned(),
            request_body: self.request_body.to_owned(),
        };
        let impl_struct = ImplStruct {
            name: self.name.to_owned(),
            path_params: self.path_params.to_owned(),
            request_body: self.request_body.to_owned(),
            query_option_names: self.query_option_names.to_owned(),
        };
        let impl_trait_endpoint = ImplTraitEndpoint {
            name: self.name.to_owned(),
            request_body: self.request_body.to_owned(),
            response_ok_body: self.response_ok_body.to_owned(),
            method: self.method.to_owned(),
            path: self.path.to_owned(),
            path_params: self.path_params.to_owned(),
        };

        let token = quote! {
            #def_struct

            #impl_struct

            #impl_trait_endpoint
        };

        tokens.append_all(token);
    }
}

//
struct DefStruct {
    name: String,
    path_params: Option<PathParams>,
    request_body: Option<Type>,
}
impl ToTokens for DefStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", &self.name);
        let path_param_fields: Vec<_> = if let Some(path_params) = &self.path_params {
            path_params
                .0
                .iter()
                .map(|(name, ty)| {
                    quote! {
                        #name: #ty,
                    }
                })
                .collect()
        } else {
            vec![]
        };

        let derive = if self.path_params.is_none() && self.request_body.is_none() {
            quote!(#[derive(Default, Debug, Clone)])
        } else {
            quote!(#[derive(Debug, Clone)])
        };

        let token = if let Some(request_body) = &self.request_body {
            quote! {
                #derive
                pub struct #name {
                    query_options: ::consul_core::api::api::QueryOptions,
                    #(#path_param_fields)*
                    body: #request_body,
                }
            }
        } else {
            quote! {
                #derive
                pub struct #name {
                    query_options: ::consul_core::api::api::QueryOptions,
                    #(#path_param_fields)*
                }
            }
        };

        tokens.append_all(token);
    }
}

//
struct ImplStruct {
    name: String,
    path_params: Option<PathParams>,
    request_body: Option<Type>,
    query_option_names: Option<QueryOptionNames>,
}
impl ToTokens for ImplStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", &self.name);
        let new_function = ImplStructNewFunction {
            path_params: self.path_params.to_owned(),
            request_body: self.request_body.to_owned(),
        };
        let parameter_functions = ImplStructParameterFunctions {
            query_option_names: self.query_option_names.to_owned(),
        };

        let token = quote! {
            impl #name {
                #new_function

                #parameter_functions
            }
        };

        tokens.append_all(token);
    }
}

struct ImplStructNewFunction {
    path_params: Option<PathParams>,
    request_body: Option<Type>,
}
impl ToTokens for ImplStructNewFunction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path_param_args: Vec<_> = if let Some(path_params) = &self.path_params {
            path_params
                .0
                .iter()
                .map(|(name, ty)| {
                    quote! {
                        #name: #ty,
                    }
                })
                .collect()
        } else {
            vec![]
        };
        let path_param_names: Vec<_> = if let Some(path_params) = &self.path_params {
            path_params
                .0
                .iter()
                .map(|(name, _)| {
                    quote! {
                        #name,
                    }
                })
                .collect()
        } else {
            vec![]
        };

        let token = if let Some(request_body) = &self.request_body {
            quote! {
                pub fn new(#(#path_param_args)* body: #request_body) -> Self {
                    Self {
                        query_options: Default::default(),
                        #(#path_param_names)*
                        body,
                    }
                }
            }
        } else {
            quote! {
                pub fn new(#(#path_param_args)*) -> Self {
                    Self {
                        query_options: Default::default(),
                        #(#path_param_names)*
                    }
                }
            }
        };

        tokens.append_all(token);
    }
}

struct ImplStructParameterFunctions {
    query_option_names: Option<QueryOptionNames>,
}
impl ToTokens for ImplStructParameterFunctions {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(query_option_names) = &self.query_option_names {
            for query_option_name in query_option_names.0.iter() {
                let (query_name, field_name, field_type) = query_option_name.field();

                let fn_name = format_ident!("set_parameter_{}", query_name);
                let field_operator = format_ident!("{}", field_name);

                let token = quote! {
                    pub fn #fn_name(&mut self, val: #field_type) -> &mut Self {
                        self.query_options.#field_operator = val;
                        self
                    }
                };

                tokens.append_all(token);
            }
        }
    }
}

//
struct ImplTraitEndpoint {
    name: String,
    request_body: Option<Type>,
    response_ok_body: TokenStream,
    method: Option<Expr>,
    path: String,
    path_params: Option<PathParams>,
}
impl ToTokens for ImplTraitEndpoint {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", &self.name);
        let request_body_token_stream = if let Some(request_body) = &self.request_body {
            quote!(#request_body)
        } else {
            quote!(())
        };
        let response_ok_body_token_stream = &self.response_ok_body;

        let method_function = ImplTraitEndpointMethodFunction {
            method: self.method.to_owned(),
        };
        let path_function = ImplTraitEndpointPathFunction {
            path: self.path.to_owned(),
            path_params: self.path_params.to_owned(),
        };

        let query_function = quote! {
            fn query(&self) -> Option<::std::collections::BTreeMap<String, Vec<String>>> {
                let map = self.query_options.params();
                if map.is_empty() {
                    None
                } else {
                    Some(map)
                }
            }
        };

        let header_function = quote! {
            fn header(&self) -> Option<::http::header::HeaderMap> {
                let map = self.query_options.header();
                if map.is_empty() {
                    None
                } else {
                    let mut header_map = ::http::header::HeaderMap::new();
                    // TODO
                    // for (k, values) in map {
                    //     for v in values {
                    //         header_map.insert(k, v.parse().unwrap());
                    //     }
                    // }
                    Some(header_map)
                }
            }
        };

        let body_function = if let Some(request_body) = &self.request_body {
            quote! {
                fn body(&self) -> Option<Self::RequestBody> {
                    Some(#request_body)
                }
            }
        } else {
            quote! {
                fn body(&self) -> Option<Self::RequestBody> {
                    None
                }
            }
        };

        let token = quote! {
            impl ::consul_api_endpoint::Endpoint for #name {
                type RequestBody = #request_body_token_stream;
                type ResponseOkBody = #response_ok_body_token_stream;

                #method_function

                #path_function

                #query_function

                #header_function

                #body_function
            }
        };

        tokens.append_all(token);
    }
}

struct ImplTraitEndpointMethodFunction {
    method: Option<Expr>,
}
impl ToTokens for ImplTraitEndpointMethodFunction {
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

struct ImplTraitEndpointPathFunction {
    path: String,
    path_params: Option<PathParams>,
}
impl ToTokens for ImplTraitEndpointPathFunction {
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
                    #path.to_owned()
                }
            }
        };

        tokens.append_all(token);
    }
}
