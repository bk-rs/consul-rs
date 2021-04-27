use golang_type_decl::{gen_json_struct_from_file, golang_type_decl_macro};

use crate::ReadableDuration;

gen_json_struct_from_file!("consul-1.9.5/api/health.go#L35-L52");

gen_json_struct_from_file!("consul-1.9.5/api/health.go#L56-L71");

pub type HealthChecks = Vec<HealthCheck>;
