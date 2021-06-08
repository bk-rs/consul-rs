use consul_api_endpoint::{endpoint, http::Method};
use consul_core::api::catalog::Node;
use golang_type::{gen_type, golang_type_macro};
use paste::paste;
use serde_json::{Map, Value};

// Endpoint Datacenters
// https://www.consul.io/api-docs/catalog#list-datacenters
// https://github.com/hashicorp/consul/blob/v1.9.5/api/catalog.go#L150
endpoint!(
    ListDatacenters,
    Method::GET,
    "/v1/catalog/datacenters",
    gen_type!("[]string")
);

// Endpoint Nodes
// https://github.com/hashicorp/consul/blob/v1.9.5/api/catalog.go#L166
// https://www.consul.io/api-docs/catalog#list-nodes
endpoint!(
    Nodes,
    ListNodes,
    gen_type!("[]*Node"),
    Method::GET,
    "/v1/catalog/nodes",
    ;
    dc = String,
    near = String,
    filter = String,
);

// Endpoint Nodes
// https://github.com/hashicorp/consul/blob/v1.9.5/api/catalog.go#L187
// https://www.consul.io/api-docs/catalog#list-services
endpoint!(
    Services,
    ListServices,
    gen_type!("map[string][]string"),
    Method::GET,
    "/v1/catalog/services",
    ;
    dc = String,
    node_meta = Map<String, Value>,
    ns = String,
);
