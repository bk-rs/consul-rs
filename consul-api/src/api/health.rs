use consul_api_endpoint::{endpoint, http::Method};
use consul_core::api::health::HealthChecks;
pub use consul_core::api::health::State;
use golang_type::{gen_type, golang_type_macro};
use paste::paste;
use serde_json::{Map, Value};

// Endpoint State
// https://www.consul.io/api-docs/health#list-checks-in-state
// https://github.com/hashicorp/consul/blob/v1.9.5/api/health.go#L349
endpoint!(
    State,
    ListChecksInState,
    gen_type!("HealthChecks"),
    Method::GET,
    "/v1/health/state/{state}",
    state => State,
    ;
    dc = String,
    near = String,
    node_meta = Map<String, Value>,
    filter = String,
    ns = String,
);
