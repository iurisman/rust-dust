#!/usr/bin/env bash

# Start (up) postgres container.

docker_dir=$(dirname $0)
docker cp $docker_dir/drop-schema.sql meter-postgres-13:drop-schema.sql
docker exec -i -e PGPASSWORD=meter meter-postgres-13  psql -U meter  -a -f drop-schema.sql

