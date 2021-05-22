use golang_type_decl::{gen_json_struct_from_file, golang_type_decl_macro};

use super::{
    agent::{AgentCheck, AgentService, AgentServiceConnectProxyConfig, ServiceKind},
    health::HealthChecks,
};

// Weights
gen_json_struct_from_file!(
    "consul-1.9.5/api/catalog.go#L8-L11",
    custom_derive = "Debug, Clone"
);

// Node
gen_json_struct_from_file!(
    "consul-1.9.5/api/catalog.go#L13-L22",
    custom_derive = "Debug, Clone"
);

// ServiceAddress
gen_json_struct_from_file!(
    "consul-1.9.5/api/catalog.go#L24-L27",
    custom_derive = "Debug, Clone"
);

// CatalogService
gen_json_struct_from_file!(
    "consul-1.9.5/api/catalog.go#L29-L50",
    custom_derive = "Debug, Clone"
);

// CatalogNode
gen_json_struct_from_file!(
    "consul-1.9.5/api/catalog.go#L52-L55",
    custom_derive = "Debug, Clone"
);

// CatalogNodeServiceList
gen_json_struct_from_file!(
    "consul-1.9.5/api/catalog.go#L57-L60",
    custom_derive = "Debug, Clone"
);

// CatalogRegistration
gen_json_struct_from_file!(
    "consul-1.9.5/api/catalog.go#L62-L73",
    custom_derive = "Debug, Clone"
);

// CatalogDeregistration
gen_json_struct_from_file!(
    "consul-1.9.5/api/catalog.go#L75-L82",
    custom_derive = "Debug, Clone"
);

// CompoundServiceName
gen_json_struct_from_file!(
    "consul-1.9.5/api/catalog.go#L84-L89",
    custom_derive = "Debug, Clone"
);

// GatewayService
gen_json_struct_from_file!(
    "consul-1.9.5/api/catalog.go#L93-L105",
    custom_derive = "Debug, Clone"
);
