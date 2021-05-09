use golang_type_decl::{gen_json_struct_from_file, golang_type_decl_macro};

use super::operator_autopilot::ReadableDuration;

// QueryOptions
gen_json_struct_from_file!(
    "consul-1.9.5/api/api.go#L81-L169";
    "MaxAge" => ReadableDuration,
    "StaleIfError" => ReadableDuration,
    "WaitTime" => ReadableDuration,
    "ctx" => (),
);
