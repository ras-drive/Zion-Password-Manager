#!/usr/bin/bash

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

cleanup () {
  docker-compose -p ci kill
  docker-compose -p ci rm -f
}

trap 'cleanup ; printf "${RED}Tests Failed For Unexpected Reasons${NC}\n"' HUP INT QUIT PIPE TERM

docker-compose -p ci build
docker-compose -p ci up -d

TEST_EXIT_CODE=$(docker wait ci_test_1)

# shellcheck disable=2181
if [ $? -ne 0 ] ; then
  printf "%sDocker Compose Failed$%s\n" "$RED" "$NC"
  exit 1
fi

if [ -z ${TEST_EXIT_CODE+x} ] || [ "$TEST_EXIT_CODE" -ne 0 ] ; then
  docker logs ci_database_1
  docker logs ci_test_1
  # printf "${RED}Tests Failed${NC} - Exit Code: $TEST_EXIT_CODE\n"
  printf "%sTests Failed%s - Exit Code: $TEST_EXIT_CODE\n" "$RED" "$NC"
else
  printf "%sTests Passed%s\n" "$GREEN" "$NC"
fi

cleanup

exit "$TEST_EXIT_CODE"
