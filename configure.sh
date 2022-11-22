#!/usr/bin/bash

cd frontend || exit
npm i

cd .. || exit

read -rp "Please input your PostgreSQL URL (example: postgres://postgres:password@localHost:5432/example): " DB_ENV

echo "DATABASE_URL=${DB_ENV}" > .env

cargo install diesel_cli --no-default-features --features postgres

cd backend/migrations || exit

echo "performing database migrations"

diesel migration run