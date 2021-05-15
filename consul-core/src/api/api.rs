use std::collections::HashMap;

use golang_type::{gen_type, golang_type_macro};
use golang_type_decl::{gen_json_struct_from_file, golang_type_decl_macro};

// QueryOptions
gen_json_struct_from_file!(
    "consul-1.9.5/api/api.go#L81-L169";
    "MaxAge" => TimeDuration,
    "StaleIfError" => TimeDuration,
    "WaitTime" => TimeDuration,
    "ctx" => (),
);

// https://golang.org/pkg/time/#Duration
pub type TimeDuration = gen_type!("int64");

// https://golang.org/pkg/net/url/#Values
pub type UrlValues = gen_type!("map[string][]string");

// https://golang.org/pkg/net/http/#Header
pub type HttpHeader = gen_type!("map[string][]string");

// https://github.com/hashicorp/consul/blob/v1.9.5/api/api.go#L686-L752
impl QueryOptions {
    pub fn params(&self) -> UrlValues {
        let mut params = HashMap::new();

        if !self.namespace.is_empty() {
            params.insert("ns".to_owned(), vec![self.namespace.to_owned()]);
        }
        if !self.datacenter.is_empty() {
            params.insert("dc".to_owned(), vec![self.datacenter.to_owned()]);
        }
        if self.allow_stale {
            params.insert("stale".to_owned(), vec![]);
        }
        if self.require_consistent {
            params.insert("consistent".to_owned(), vec![]);
        }
        if self.wait_index != 0 {
            params.insert("index".to_owned(), vec![format!("{}", self.wait_index)]);
        }
        if self.wait_time != 0 {
            params.insert("wait".to_owned(), vec![dur_to_msec(self.wait_time)]);
        }
        if !self.wait_hash.is_empty() {
            params.insert("hash".to_owned(), vec![self.wait_hash.to_owned()]);
        }

        params
    }

    pub fn header(&self) -> HttpHeader {
        let mut header = HashMap::new();

        if !self.token.is_empty() {
            header.insert("X-Consul-Token".to_owned(), vec![self.token.to_owned()]);
        }

        header
    }
}

// https://github.com/hashicorp/consul/blob/v1.9.5/api/api.go#L754-L764
fn dur_to_msec(_dur: TimeDuration) -> String {
    unimplemented!()
}
