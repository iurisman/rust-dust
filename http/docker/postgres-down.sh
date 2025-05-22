#!/usr/bin/env bash

docker_dir=$(dirname $0)

# Stop and destroy (down) postgres container and dependent volumes.
docker compose -f $docker_dir/rust-dust-http-pg13.yml down -v

