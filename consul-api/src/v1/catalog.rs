use golang_type::{gen_type, golang_type_macro};
use golang_type_decl::{gen_json_struct_from_file, golang_type_decl_macro};

use crate::endpoint::http::Method;

use super::{
    agent::{AgentCheck, AgentService, AgentServiceConnectProxyConfig, ServiceKind},
    health::HealthChecks,
};

// Weights
gen_json_struct_from_file!("consul-1.9.5/api/catalog.go#L8-L11");

// Node
gen_json_struct_from_file!("consul-1.9.5/api/catalog.go#L13-L22");

// ServiceAddress
gen_json_struct_from_file!("consul-1.9.5/api/catalog.go#L24-L27");

// CatalogService
gen_json_struct_from_file!("consul-1.9.5/api/catalog.go#L29-L50");

// CatalogNode
gen_json_struct_from_file!("consul-1.9.5/api/catalog.go#L52-L55");

// CatalogNodeServiceList
gen_json_struct_from_file!("consul-1.9.5/api/catalog.go#L57-L60");

// CatalogRegistration
gen_json_struct_from_file!("consul-1.9.5/api/catalog.go#L62-L73");

// CatalogDeregistration
gen_json_struct_from_file!("consul-1.9.5/api/catalog.go#L75-L82");

// CompoundServiceName
gen_json_struct_from_file!("consul-1.9.5/api/catalog.go#L84-L89");

// GatewayService
gen_json_struct_from_file!("consul-1.9.5/api/catalog.go#L93-L105");

// Endpoint Datacenters
// https://github.com/hashicorp/consul/blob/v1.9.5/api/catalog.go#L150
endpoint!(
    Datacenters,
    Method::GET,
    "/v1/catalog/datacenters",
    gen_type!("[]string")
);

// Endpoint Nodes
// https://github.com/hashicorp/consul/blob/v1.9.5/api/catalog.go#L166
endpoint!(
    Nodes,
    Method::GET,
    "/v1/catalog/nodes",
    gen_type!("[]*Node")
);

// Endpoint Nodes
// https://github.com/hashicorp/consul/blob/v1.9.5/api/catalog.go#L187
endpoint!(
    Services,
    Method::GET,
    "/v1/catalog/services",
    gen_type!("map[string][]string")
);
