#!/usr/bin/env bash

# Start (up) postgres container.

docker_dir=$(dirname $0)
# Create 'meter' user and database.
docker exec -i rust-dust-http-pg13 psql -U postgres <<EOF
  drop database if exists rust_dust;
  drop user if exists dust;
  create database rust_dust;
  \connect rust_dust
  create user dust password 'dust';
EOF

docker cp $docker_dir/create-schema.sql rust-dust-http-pg13:create-schema.sql
docker exec -i -e PGPASSWORD=dust rust-dust-http-pg13  psql -U dust  -a -f create-schema.sql

