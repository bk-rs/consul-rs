use consul_api_endpoint::{endpoint, http::Method};
use consul_core::api::catalog::Node;
use golang_type::{gen_type, golang_type_macro};

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
// https://www.consul.io/api-docs/catalog#list-nodes
// https://github.com/hashicorp/consul/blob/v1.9.5/api/catalog.go#L166
endpoint!(
    ListNodes,
    Method::GET,
    "/v1/catalog/nodes",
    { dc, near, node_meta, filter, },
    gen_type!("[]*Node"),
);

// // Endpoint Nodes
// // https://www.consul.io/api-docs/catalog#list-services
// // https://github.com/hashicorp/consul/blob/v1.9.5/api/catalog.go#L187
endpoint!(
    ListServices,
    Method::GET,
    "/v1/catalog/services",
    { dc, node_meta, filter, ns, },
    gen_type!("map[string][]string"),
);
