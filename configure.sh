#!/usr/bin/bash

read -e -p "Enter your MongoDB URI: " MONGODB_URI
echo "MONGODB_URI=$MONGODB_URI" > .env
