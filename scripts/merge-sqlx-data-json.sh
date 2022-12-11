#!/usr/bin/env sh
rm -rf target/sqlx
touch src/*.rs
env -u DATABASE_URL SQLX_OFFLINE=false cargo check --workspace
if [ $? -eq 0 ]; then
    jq -s '{"db": "PostgreSQL"} + (INDEX(.hash)|to_entries|sort_by(.key)|from_entries)' target/sqlx/query-*.json > sqlx-data.json
else
    echo "cargo check failed, did you start the database by running ./scripts/start_db.sh?"
    exit 1
fi