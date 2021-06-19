use consul_api_endpoint::{consul_api_endpoint_macro, query_endpoint};
use consul_core::api::health::HealthChecks;
pub use consul_core::api::health::State;
use golang_type::{gen_type, golang_type_macro};

// Endpoint State
// https://www.consul.io/api-docs/health#list-checks-in-state
// https://github.com/hashicorp/consul/blob/v1.9.5/api/health.go#L349
query_endpoint!(
    ListChecksInState,
    "/v1/health/state/{state}",
    { state => State, },
    { dc, near, node_meta, filter, ns, },
    gen_type!("HealthChecks"),
);
