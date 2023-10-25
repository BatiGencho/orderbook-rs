#!/bin/bash

sudo docker-compose -f orderbook-api/docker-compose.yml up -d --build $@
#sudo docker-compose -f ../orderbook-api/docker-compose.yml  --env-file ../Laravel-Core-API/.env up -d --build $@