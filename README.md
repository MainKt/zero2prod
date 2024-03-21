# zero2prod
Follow along of 'Zero to Production In Rust' by Luca Palmieri. This repository contains the code for a newsletter API written in Rust.

## Pre-requisites
You'll need to install:
- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)
- [PostgreSQL](https://www.postgresql.org/download/) for `psql` CLI client
- [sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#install)
- [doctl](https://docs.digitalocean.com/reference/doctl/how-to/install/) the official DigitalOcean CLI

## Local Development
To build the application locally for development:
1. Launch a (migrated) Postgres database via Docker:
```bash
./scripts/init_db.sh
```
To migrate without launching a Postgres instance:
```bash
SKIP_DOCKER=true ./scripts/init_db.sh
```
2. Launch the web server:
```bash
cargo run
```
For the inner development loop, use:
```bash
cargo watch -x check -x test -x run
```

## Production
To deploy the application to Digital Ocean:
1. Authenticate with Digital Ocean:
```bash
doctl auth init
```
2. Deploy the application using the provided `spec.yaml` file:
```bash
doctl apps create --spec spec.yaml
```
3. Migrate the database by running the following command with your Digital Ocean DB connection string:
```bash
DATABASE_URL=YOUR-DIGITAL-OCEAN-DB-CONNECTION-STRING sqlx migrate run
```
To apply changes to Digital Ocean every time you modify `spec.yaml`:
- First, grab your app identifier via:
```bash
doctl apps list --format ID
```
- Then, run the following command with your app ID:
```bash
doctl apps update $APP_ID --spec spec.yaml
``` 
