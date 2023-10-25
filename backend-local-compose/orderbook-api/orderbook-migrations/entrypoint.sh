#!/bin/bash

set -x

diesel database reset --database-url $DATABASE_URL/orders --migration-dir ./migrations
diesel migration run --database-url $DATABASE_URL/orders --migration-dir ./migrations
