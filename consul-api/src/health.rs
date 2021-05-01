use golang_type_decl::{
    gen_json_struct_from_file, gen_type_alias_from_file, golang_type_decl_macro,
};

use crate::operator_autopilot::ReadableDuration;

// HealthCheck
gen_json_struct_from_file!("consul-1.9.5/api/health.go#L35-L52");

// HealthCheckDefinition
gen_json_struct_from_file!(
    "consul-1.9.5/api/health.go#L56-L71";
    "IntervalDuration" => ReadableDuration,
    "TimeoutDuration" => ReadableDuration,
    "DeregisterCriticalServiceAfterDuration" => ReadableDuration,
);

// HealthChecks
gen_type_alias_from_file!("consul-1.9.5/api/health.go#L167");
