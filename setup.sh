#!/bin/sh
docker-compose -f docker-compose-dev.yml &&
    diesel setup --database-url "postgres://union:union@localhost:6969/union-db"
