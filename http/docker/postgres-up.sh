#!/usr/bin/env bash

# Start (up) postgres container.

docker_dir=$(dirname $0)
$docker_dir/postgres-down.sh
docker compose -f $docker_dir/rust-dust-http-pg13.yml up -d
