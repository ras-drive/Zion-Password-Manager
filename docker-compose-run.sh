#!/usr/bin/bash

RED='\033[0;31m'
NC='\033[0m'

cleanup () {
  docker-compose -p zion kill
  docker-compose -p zion rm -f
}

trap 'cleanup ; printf "${RED}Shutting down${NC}\n"' HUP INT QUIT PIPE TERM

docker-compose -p zion build && \
    docker-compose -p zion up database server
