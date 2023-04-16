#!/bin/sh
docker-compose -f docker-compose-dev.yml up -d &&
    sleep 1 &&
    diesel setup --database-url "postgres://derive:cloud@localhost:6969/derive-cloud"
