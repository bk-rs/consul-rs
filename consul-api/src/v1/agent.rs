use std::collections::HashMap;

use golang_type_decl::{
    gen_json_struct_from_file, gen_type_alias_from_file, golang_type_decl_macro,
};
use serde_json::Value;

use super::{
    catalog::ServiceAddress,
    health::HealthCheckDefinition,
    proto::pbservice::service::{ExposeConfig, MeshGatewayConfig, Upstream},
};

// ServiceKind
gen_type_alias_from_file!("consul-1.9.5/api/agent.go#L13");

// AgentCheck
gen_json_struct_from_file!("consul-1.9.5/api/agent.go#L55-L67");

// AgentWeights
gen_json_struct_from_file!("consul-1.9.5/api/agent.go#L70-L73");

// AgentService
gen_json_struct_from_file!("consul-1.9.5/api/agent.go#L76-L98");

// AgentServiceConnect
gen_json_struct_from_file!(
    "consul-1.9.5/api/agent.go#L108-L111";
    "SidecarService" => { "box_type": true }
);

// AgentServiceConnectProxyConfig
gen_json_struct_from_file!(
    "consul-1.9.5/api/agent.go#L115-L124";
    "Config" => HashMap<String, Value>
);

// AgentServiceRegistration
gen_json_struct_from_file!("consul-1.9.5/api/agent.go#L251-L267");

// AgentServiceCheck
gen_json_struct_from_file!("consul-1.9.5/api/agent.go#L288-L319");

// AgentServiceChecks
gen_type_alias_from_file!("consul-1.9.5/api/agent.go#L320");
