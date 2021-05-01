extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/proto/pbservice")
        .inputs(&[
            "consul-1.9.5/proto/pbcommon/common.proto",
            "consul-1.9.5/proto/pbservice/healthcheck.proto",
            "consul-1.9.5/proto/pbservice/service.proto",
            "gogo_protobuf-65acae22fc9d/gogoproto/gogo.proto",
        ])
        .include("consul-1.9.5")
        .include("gogo_protobuf-65acae22fc9d")
        .customize(Customize {
            serde_derive: Some(true),
            ..Default::default()
        })
        .run()
        .unwrap();
}
