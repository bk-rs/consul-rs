#!/usr/bin/env bash

set -ex

# Prerequire
# ./../consul_bin/download.sh 1.9.5

# RUST_BACKTRACE=full ./tests/run_integration_tests.sh
# RUST_LOG=trace ./tests/run_integration_tests.sh

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

run="${script_path_root}../../consul_bin/run.sh"

# https://unix.stackexchange.com/questions/55913/whats-the-easiest-way-to-find-an-unused-local-port
read LOWERPORT UPPERPORT < /proc/sys/net/ipv4/ip_local_port_range
http_port=$(comm -23 <(seq $LOWERPORT $UPPERPORT | sort) <(ss -Htan | awk '{print $4}' | cut -d':' -f2 | sort -u) | shuf | head -n 1)

export CONSUL_HTTP_URL="http://127.0.0.1:${http_port}"

${run} 1.9.5 ${http_port} "cd ${script_path_root}..; cargo test -p consul-api --features _integration_tests -- --nocapture"
