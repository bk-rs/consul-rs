use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    Error as SynError, Ident, LitStr, Token,
};

// https://github.com/hashicorp/consul/blob/v1.9.5/api/api.go#L80-L169
#[derive(PartialEq, Clone)]
pub enum QueryOptionName {
    Namespace,
    Datacenter,
    AllowStale,
    RequireConsistent,
    UseCache,
    MaxAge,
    StaleIfError,
    WaitIndex,
    WaitHash,
    WaitTime,
    Token,
    Near,
    NodeMeta,
    RelayFactor,
    LocalOnly,
    Connect,
    Filter,
}
impl QueryOptionName {
    pub fn field(&self) -> (&str, TokenStream) {
        match self {
            QueryOptionName::Namespace => ("ns", quote!(::std::string::String)),
            QueryOptionName::Datacenter => todo!(),
            QueryOptionName::AllowStale => todo!(),
            QueryOptionName::RequireConsistent => todo!(),
            QueryOptionName::UseCache => todo!(),
            QueryOptionName::MaxAge => todo!(),
            QueryOptionName::StaleIfError => todo!(),
            QueryOptionName::WaitIndex => todo!(),
            QueryOptionName::WaitHash => todo!(),
            QueryOptionName::WaitTime => todo!(),
            QueryOptionName::Token => todo!(),
            QueryOptionName::Near => todo!(),
            QueryOptionName::NodeMeta => todo!(),
            QueryOptionName::RelayFactor => todo!(),
            QueryOptionName::LocalOnly => todo!(),
            QueryOptionName::Connect => todo!(),
            QueryOptionName::Filter => todo!(),
        }
    }
}

#[derive(Default, Clone)]
pub struct QueryOptionNames(pub Vec<QueryOptionName>);

impl Parse for QueryOptionNames {
    fn parse(input: ParseStream) -> Result<Self, SynError> {
        let mut inner = vec![];

        loop {
            let query_option_name = if input.peek(Ident) {
                input.parse::<Ident>()?
            } else {
                format_ident!("{}", input.parse::<LitStr>()?.value())
            };
            let name = match &query_option_name {
                s if s == "ns" || s == "namespace" => QueryOptionName::Namespace,
                s if s == "dc" || s == "datacenter" => QueryOptionName::Datacenter,
                s if s == "stale" => QueryOptionName::AllowStale,
                s if s == "consistent" => QueryOptionName::RequireConsistent,
                s if s == "cached" => QueryOptionName::UseCache,
                s if s == "max-age" || s == "max_age" => QueryOptionName::MaxAge,
                s if s == "stale-if-error" || s == "stale_if_error" => {
                    QueryOptionName::StaleIfError
                }
                s if s == "index" => QueryOptionName::WaitIndex,
                s if s == "wait" => QueryOptionName::WaitTime,
                s if s == "hash" => QueryOptionName::WaitHash,
                s if s == "token" => QueryOptionName::Token,
                s if s == "near" => QueryOptionName::Near,
                s if s == "node-meta" || s == "node_meta" => QueryOptionName::NodeMeta,
                s if s == "relay-factor" || s == "relay_factor" => QueryOptionName::RelayFactor,
                s if s == "local-only" || s == "local_only" => QueryOptionName::LocalOnly,
                s if s == "connect" => QueryOptionName::Connect,
                s if s == "filter" => QueryOptionName::Filter,
                _ => {
                    let err = format!("unknown query option name: {}", &query_option_name);
                    return Err(SynError::new_spanned(query_option_name, err));
                }
            };

            if inner.contains(&name) {
                let err = format!("duplicate query option name: {}", &query_option_name);
                return Err(SynError::new_spanned(query_option_name, err));
            }
            inner.push(name);

            input.parse::<Token![,]>()?;

            if !(input.peek(Ident) || input.peek(LitStr)) {
                break;
            }
        }

        Ok(Self(inner))
    }
}
