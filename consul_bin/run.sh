#!/usr/bin/env bash

set -ex

# ./run.sh 1.10.0-beta1 8500 "sleep 3"

version="${1:-1.9.5}"
http_port=$2
callback=$3

if [ -z "$http_port" ]
then
    exit 91
fi
if [ -z "$callback" ]
then
    exit 92
fi

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

version_root_path="${script_path_root}${version}/"

bin="${version_root_path}consul"

# 
# https://www.consul.io/docs/install/ports
# 
# https://unix.stackexchange.com/questions/55913/whats-the-easiest-way-to-find-an-unused-local-port
read LOWERPORT UPPERPORT < /proc/sys/net/ipv4/ip_local_port_range
server_port=$(comm -23 <(seq $LOWERPORT $UPPERPORT | sort) <(ss -Htan | awk '{print $4}' | cut -d':' -f2 | sort -u) | shuf | head -n 1)
serf_lan_port=$(comm -23 <(seq $LOWERPORT $UPPERPORT | sort) <(ss -Htan | awk '{print $4}' | cut -d':' -f2 | sort -u) | shuf | head -n 1)
serf_wan_port=$(comm -23 <(seq $LOWERPORT $UPPERPORT | sort) <(ss -Htan | awk '{print $4}' | cut -d':' -f2 | sort -u) | shuf | head -n 1)
dns_port=$(comm -23 <(seq $LOWERPORT $UPPERPORT | sort) <(ss -Htan | awk '{print $4}' | cut -d':' -f2 | sort -u) | shuf | head -n 1)

workdir=$(mktemp -d)

mkdir -p "${workdir}/log"
log_file="${workdir}/log/consul.log"

mkdir -p "${workdir}/run"
pid_file="${workdir}/run/consul.pid"

cleanup() {
  test -f "${pid_file}" && kill $(cat "${pid_file}")
  test -f "${log_file}" && cat "${log_file}"
  rm -rf "${workdir}"
}
trap cleanup EXIT

${bin} agent -dev -ui \
    -server-port=${server_port} \
    -serf-lan-port=${serf_lan_port} \
    -serf-wan-port=${serf_wan_port} \
    -http-port=${http_port} \
    -https-port=-1 -grpc-port=-1 \
    -log-file="${log_file}" \
    -log-level="err" \
    -pid-file="${pid_file}" &


sleep 2

echo "callback running..."

bash -c "${callback}"
