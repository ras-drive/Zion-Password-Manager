#!/usr/bin/bash

# checks if rust toolchain is installed
if ! command -v rustup &> /dev/null
then
    echo "please install the rust toolchain to proceed"
    exit
else
    echo "rust toolchain is installed"
fi

# checks if node is installed and at least version 16
if ! command -v node &> /dev/null
then
    echo "please install node to proceed"
    exit
else
    nodeVersion=$(node -v)
    nodeVersion=${nodeVersion:1:2}

    if [ "$nodeVersion" -gt 15 ]
    then
        echo "node is at least version 16"
    else
        echo "node must be at least version 16, please use nvm to install a newer version"
        exit
    fi
fi

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