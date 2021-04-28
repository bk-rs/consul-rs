use std::collections::HashMap;

use golang_type_decl::{gen_json_struct_from_file, golang_type_decl_macro};
use serde_json::Value;

use crate::{
    catalog::ServiceAddress,
    health::HealthCheckDefinition,
    proto::pbservice::service::{ExposeConfig, MeshGatewayConfig, Upstream},
};

// L13
pub type ServiceKind = String;

gen_json_struct_from_file!("consul-1.9.5/api/agent.go#L55-L67");

gen_json_struct_from_file!("consul-1.9.5/api/agent.go#L70-L73");

gen_json_struct_from_file!("consul-1.9.5/api/agent.go#L76-L98");

gen_json_struct_from_file!(
    "consul-1.9.5/api/agent.go#L108-L111";
    "SidecarService" => Box<AgentServiceRegistration>
);

gen_json_struct_from_file!(
    "consul-1.9.5/api/agent.go#L115-L124";
    "Config" => HashMap<String, Value>
);

gen_json_struct_from_file!("consul-1.9.5/api/agent.go#L251-L267");

gen_json_struct_from_file!("consul-1.9.5/api/agent.go#L288-L319");

// L320
pub type AgentServiceChecks = Vec<AgentServiceCheck>;
