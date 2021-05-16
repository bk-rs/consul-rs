use std::{collections::HashMap, time::Duration};

use golang_type::{gen_type, golang_type_macro};
use golang_type_decl::{gen_json_struct_from_file, golang_type_decl_macro};

// QueryOptions
gen_json_struct_from_file!(
    "consul-1.9.5/api/api.go#L81-L169";
    "MaxAge" => Duration,
    "StaleIfError" => Duration,
    "WaitTime" => Duration,
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
        if self.wait_time.as_secs() != 0 {
            params.insert("wait".to_owned(), vec![dur_to_msec(self.wait_time)]);
        }
        if !self.wait_hash.is_empty() {
            params.insert("hash".to_owned(), vec![self.wait_hash.to_owned()]);
        }
        if !self.near.is_empty() {
            params.insert("near".to_owned(), vec![self.near.to_owned()]);
        }
        if !self.filter.is_empty() {
            params.insert("filter".to_owned(), vec![self.filter.to_owned()]);
        }
        if !self.node_meta.is_empty() {
            for (meta_k, meta_v) in &self.node_meta {
                let v = params.entry("node-meta".to_owned()).or_default();
                v.push(format!("{}:{}", meta_k, meta_v));
            }
        }
        if self.relay_factor != 0 {
            params.insert(
                "relay-factor".to_owned(),
                vec![format!("{}", self.relay_factor)],
            );
        }
        if self.local_only {
            params.insert(
                "local-only".to_owned(),
                vec![format!("{}", self.local_only)],
            );
        }
        if self.connect {
            params.insert("connect".to_owned(), vec!["true".to_owned()]);
        }
        if self.use_cache && !self.require_consistent {
            params.insert("cached".to_owned(), vec!["".to_owned()]);
        }

        params
    }

    pub fn header(&self) -> HttpHeader {
        let mut header = HashMap::new();

        if !self.token.is_empty() {
            header.insert("X-Consul-Token".to_owned(), vec![self.token.to_owned()]);
        }
        if self.use_cache && !self.require_consistent {
            let mut cc = vec![];
            if self.max_age.as_secs() > 0 {
                cc.push(format!("max-age={}", self.max_age.as_secs()))
            }
            if self.stale_if_error.as_secs() > 0 {
                cc.push(format!("stale-if-error={}", self.stale_if_error.as_secs()))
            }
            if !cc.is_empty() {
                header.insert("Cache-Control".to_owned(), vec![cc.join(", ")]);
            }
        }

        header
    }
}

// https://github.com/hashicorp/consul/blob/v1.9.5/api/api.go#L754-L764
fn dur_to_msec(_dur: Duration) -> String {
    unimplemented!()
}
