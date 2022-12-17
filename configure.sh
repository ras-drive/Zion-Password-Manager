#!/usr/bin/bash

cd frontend || exit
npm i

cd .. || exit

read -rp "Please input your PostgreSQL URL (example: postgres://postgres:password@localHost:5432/example): " DB_ENV

read -rp "If you have a test database please enter the URL, otherwise leave it blank: " TEST_DB_ENV

echo "DATABASE_URL=${DB_ENV}" > .env

if [ "$TEST_DB_ENV" = "" ];
then
    echo "TEST_DATABASE_URL=${DB_ENV}" >> .env
else
    echo "TEST_DATABASE_URL=${TEST_DB_ENV}" >> .env
fi

cargo install diesel_cli --no-default-features --features postgres

cd backend/migrations || exit

echo "performing database migrations"

diesel migration run