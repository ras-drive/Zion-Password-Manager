#!/usr/bin/bash

cd frontend | exit
npm i

cd .. | exit

read -p "Please input your PostgreSQL URL (example: postgres://postgres:password@localHost:5432/example): " DB_ENV

echo "DATABASE_URL=${DB_ENV}" > .env