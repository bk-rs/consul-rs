## Dev

```
wget https://github.com/hashicorp/consul/archive/refs/tags/v1.9.5.tar.gz -O consul-v1.9.5.tar.gz
tar -zxvf consul-v1.9.5.tar.gz
```

```
# https://github.com/hashicorp/consul/blob/v1.9.5/vendor/modules.txt#L151

wget https://github.com/gogo/protobuf/archive/65acae22fc9d1fe290b33faa2bd64cdc20a463a0.tar.gz -O gogo_protobuf-65acae22fc9d.tar.gz
tar -zxvf gogo_protobuf-65acae22fc9d.tar.gz
mv protobuf-65acae22fc9d1fe290b33faa2bd64cdc20a463a0 gogo_protobuf-65acae22fc9d
```

```
cargo clippy --all-features -- -D clippy::all
cargo +nightly clippy --all-features -- -D clippy::all

cargo fmt -- --check

cargo build-all-features
cargo test-all-features -- --nocapture
```
