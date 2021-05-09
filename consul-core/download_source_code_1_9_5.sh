#!/usr/bin/env bash

set -ex

# ./download_source_code_1_9_5.sh

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

rm -rf "${script_path_root}consul-v1.9.5.tar.gz"
wget -O "${script_path_root}consul-v1.9.5.tar.gz" "https://github.com/hashicorp/consul/archive/refs/tags/v1.9.5.tar.gz"
tar -zxvf "${script_path_root}consul-v1.9.5.tar.gz" -C "${script_path_root}"

# https://github.com/hashicorp/consul/blob/v1.9.5/vendor/modules.txt#L151

rm -rf "${script_path_root}gogo_protobuf-65acae22fc9d.tar.gz"
wget -O "${script_path_root}gogo_protobuf-65acae22fc9d.tar.gz" "https://github.com/gogo/protobuf/archive/65acae22fc9d1fe290b33faa2bd64cdc20a463a0.tar.gz"
mkdir -p "${script_path_root}gogo_protobuf-65acae22fc9d"
tar -zxvf "${script_path_root}gogo_protobuf-65acae22fc9d.tar.gz" -C "${script_path_root}gogo_protobuf-65acae22fc9d" --strip-components 1
