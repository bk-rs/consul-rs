use std::collections::HashMap;

use golang_type_decl::{
    gen_json_struct_from_file, gen_type_alias_from_file, golang_type_decl_macro,
};
use serde_json::Value;

use crate::proto::pbservice::service::{ExposeConfig, MeshGatewayConfig, Upstream};

use super::{catalog::ServiceAddress, health::HealthCheckDefinition};

// ServiceKind
gen_type_alias_from_file!("consul-1.9.5/api/agent.go#L13");

// AgentCheck
gen_json_struct_from_file!(
    "consul-1.9.5/api/agent.go#L55-L67",
    custom_derive = "Debug, Clone"
);

// AgentWeights
gen_json_struct_from_file!(
    "consul-1.9.5/api/agent.go#L70-L73",
    custom_derive = "Debug, Clone"
);

// AgentService
gen_json_struct_from_file!(
    "consul-1.9.5/api/agent.go#L76-L98",
    custom_derive = "Debug, Clone"
);

// AgentServiceConnect
gen_json_struct_from_file!(
    "consul-1.9.5/api/agent.go#L108-L111",
    custom_derive = "Debug, Clone",
    ;
    "SidecarService" => { "box_type": true }
);

// AgentServiceConnectProxyConfig
gen_json_struct_from_file!(
    "consul-1.9.5/api/agent.go#L115-L124",
    custom_derive = "Debug, Clone",
    ;
    "Config" => HashMap<String, Value>
);

// AgentServiceRegistration
gen_json_struct_from_file!(
    "consul-1.9.5/api/agent.go#L251-L267",
    custom_derive = "Debug, Clone"
);

// ServiceRegisterOpts
gen_json_struct_from_file!(
    "consul-1.9.5/api/agent.go#L270-L275",
    disable_derive_serde_ser = true,
    disable_derive_serde_de = true,
    custom_derive = "Default, Debug, Clone"
);

// AgentServiceCheck
gen_json_struct_from_file!(
    "consul-1.9.5/api/agent.go#L288-L319",
    custom_derive = "Debug, Clone"
);

// AgentServiceChecks
gen_type_alias_from_file!("consul-1.9.5/api/agent.go#L320");
