#!/bin/bash

cd backend/migrations || exit
diesel migration run
cd ../.. || exit

make run