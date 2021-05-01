#!/usr/bin/env bash

set -ex

# ./download.sh 1.10.0-beta1

version="${1:-1.9.5}"

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

version_root_path="${script_path_root}${version}/"

mkdir -p "${version_root_path}"

rm -rf "${version_root_path}consul_${version}_linux_amd64.zip"
wget -O "${version_root_path}consul_${version}_linux_amd64.zip" "https://releases.hashicorp.com/consul/${version}/consul_${version}_linux_amd64.zip"

unzip -o "${version_root_path}consul_${version}_linux_amd64.zip" -d "${version_root_path}"
