#!/bin/sh
docker-compose -f docker-compose-dev.yml up -d &&
    sleep 1 &&
    diesel setup --database-url "postgres://lifetree:lifetree@localhost:6969/lifetree"
