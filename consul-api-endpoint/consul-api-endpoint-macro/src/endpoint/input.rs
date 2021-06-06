use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Error as SynError, Expr, Ident, LitStr, Token, Type,
};

use super::{path_params::PathParams, query_option_names::QueryOptionNames};

pub struct Input {
    pub name: String,
    pub method: Option<Expr>,
    pub path: String,
    pub path_params: Option<PathParams>,
    pub query_option_names: Option<QueryOptionNames>,
    pub request_body: Option<Type>,
    pub response_ok_body: TokenStream,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self, SynError> {
        let mut name = String::new();
        let mut method = None;
        let mut path = String::new();
        let mut path_params = None;
        let mut query_option_names = None;
        let mut request_body = None;
        let mut response_ok_body = quote!(());

        while !input.is_empty() {
            let key = input.parse::<Ident>()?;
            input.parse::<Token![=]>()?;

            match key {
                k if k == "name" => {
                    let v = input.parse::<LitStr>()?;
                    name = v.value();
                    if name.is_empty() {
                        return Err(SynError::new(v.span(), "name empty"));
                    }
                    input.parse::<Token![,]>()?;
                }
                k if k == "method" => {
                    method = Some(input.parse::<Expr>()?);
                    input.parse::<Token![,]>()?;
                }
                k if k == "path" => {
                    let v = input.parse::<LitStr>()?;
                    path = v.value();
                    if path.is_empty() {
                        return Err(SynError::new(v.span(), "path empty"));
                    }
                    input.parse::<Token![,]>()?;
                }
                k if k == "path_params" => {
                    path_params = Some(input.parse()?);
                    input.parse::<Token![,]>()?;
                }
                k if k == "query_option_names" => {
                    query_option_names = Some(input.parse()?);
                    input.parse::<Token![,]>()?;
                }
                k if k == "request_body" => {
                    request_body = Some(input.parse()?);
                    input.parse::<Token![,]>()?;
                }
                k if k == "response_ok_body" => {
                    response_ok_body = input.parse::<Type>()?.to_token_stream();
                    input.parse::<Token![,]>()?;
                }
                _ => {}
            }
        }

        if name.is_empty() {
            return Err(SynError::new(input.span(), "name missing"));
        }
        if path.is_empty() {
            return Err(SynError::new(input.span(), "path missing"));
        }

        Ok(Self {
            name,
            method,
            path,
            path_params,
            query_option_names,
            request_body,
            response_ok_body,
        })
    }
}
