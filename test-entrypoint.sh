#!/bin/bash

cd backend/migrations
diesel migration run
cd ../..

make test
