use golang_type_decl::gen_json_struct_from_file;

use crate::proto::pbservice::service::{ExposeConfig, MeshGatewayConfig, Upstream};

gen_json_struct_from_file!("consul-1.9.5/api/agent.go#L115-L124");
