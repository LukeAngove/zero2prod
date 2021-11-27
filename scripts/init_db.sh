#!/usr/bin/env bash

set -x
set -eo pipefail

if ! [ -x "$(command -v sqlx)" ]
then
    echo >&2 "Error: `sqlx` is not installed."
    echo >&2 "Use:"
    echo >&2 "  cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
    echo >&2 "to install it."
    exit 1
fi

DB_CONTAINER_NAME="zero2prod-postgres"
DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

if [ ! "$(podman ps --quiet --format name=${DB_CONTAINER_NAME})" ]
then
    # Container only used for testing, remove after stopped.
    podman run \
        --detach \
        --rm \
        --env POSTGRES_USER=${DB_USER} \
        --env POSTGRES_PASSWORD=${DB_PASSWORD} \
        --env POSTGRES_DB=${DB_NAME} \
        --publish "${DB_PORT}":5432 \
        --name "${DB_CONTAINER_NAME}" \
        postgres \
        postgres -N 1000
fi

export PGPASSWORD="${DB_PASSWORD}"
until podman exec "${DB_CONTAINER_NAME}" psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q';
do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}"
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
