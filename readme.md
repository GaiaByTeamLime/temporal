# Temporal microservice

This microservice is responsible for logging time series data. All the sensors connect to this microservice and send their data based on a clock (once an hour). This service connects with the auth microservice to check provided tokens. Firebase will connect once an hour and pull last hourly data to put this in firestore, since this is much easier to read from in the app.

## Developing

Install nix, then run `nix develop` to pull all dependencies and start developing. To start a database, make sure you have docker installed and run `./scripts/start_db.sh`. This will start a temporary container using the model defined in `./scripts/postgres-schema.sql` and seeds this using data from `./scripts/postgres-seeddata.sql`. It will put the correct connection string in `.env`. No need to source it, dotenv will read directly from the file.

## Releasing

First, make sure `sqlx-data.json` is up to date by starting up the development environment, and running `./scripts/merge-sqlx-data-json.sh`. Then, run `docker build --progress=plain -t temporal:latest -t temporal:0.1.0 .` to build an image, or run `docker compose up --build` to do the same but also start a database.