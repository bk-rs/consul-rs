use consul_api_endpoint::{consul_api_endpoint_macro, query_endpoint};
use consul_core::api::catalog::Node;
use golang_type::{gen_type, golang_type_macro};

// Endpoint Datacenters
// https://www.consul.io/api-docs/catalog#list-datacenters
// https://github.com/hashicorp/consul/blob/v1.9.5/api/catalog.go#L150
query_endpoint!(
    ListDatacenters,
    "/v1/catalog/datacenters",
    gen_type!("[]string")
);

// Endpoint Nodes
// https://www.consul.io/api-docs/catalog#list-nodes
// https://github.com/hashicorp/consul/blob/v1.9.5/api/catalog.go#L166
query_endpoint!(
    ListNodes,
    "/v1/catalog/nodes",
    { dc, near, node_meta, filter, },
    gen_type!("[]*Node"),
);

// // Endpoint Services
// // https://www.consul.io/api-docs/catalog#list-services
// // https://github.com/hashicorp/consul/blob/v1.9.5/api/catalog.go#L187
query_endpoint!(
    ListServices,
    "/v1/catalog/services",
    { dc, node_meta, filter, ns, },
    gen_type!("map[string][]string"),
);
