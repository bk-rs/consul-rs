use std::fmt;

use golang_type_decl::{
    gen_json_struct_from_file, gen_type_alias_from_file, golang_type_decl_macro,
};
use serde_json::Value;

use super::{api::TimeDuration, operator_autopilot::ReadableDuration};

// HealthCheck
gen_json_struct_from_file!(
    "consul-1.9.5/api/health.go#L35-L52",
    custom_derive = "Debug, Clone",
    ;
    // TODO, {"Definition": {}}
    "Definition" => Value,
);

// HealthCheckDefinition
gen_json_struct_from_file!(
    "consul-1.9.5/api/health.go#L56-L71",
    custom_derive = "Debug, Clone",
    ;
    "IntervalDuration" => TimeDuration,
    "TimeoutDuration" => TimeDuration,
    "DeregisterCriticalServiceAfterDuration" => TimeDuration,
);

// HealthChecks
gen_type_alias_from_file!("consul-1.9.5/api/health.go#L167");

// https://github.com/hashicorp/consul/blob/v1.9.5/api/health.go#L350-L357
#[derive(Debug, Clone)]
pub enum State {
    Any,
    Passing,
    Warning,
    Critical,
}
impl Default for State {
    fn default() -> Self {
        Self::Any
    }
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Any => f.write_str("any"),
            Self::Passing => f.write_str("passing"),
            Self::Warning => f.write_str("warning"),
            Self::Critical => f.write_str("critical"),
        }
    }
}
