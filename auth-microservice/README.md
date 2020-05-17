# Auth microservice

Everything is set up to run via docker-compose :)

## Configuration
You can use either config.toml or environment variables. Environment variables need to have prefix `AUTH_MICRO_`.

Supported variables:

* port = port where the service should listen (e.g. `8512`)
* database_url - URL of PostgreSQL database to connect to (e.g. `postgresql://authmicro@localhost:5432/authmicro`)

## Used ports

* 6512 & 9512 - Grafana
* 3512 - Prometheus (more efficient replacement for ElasticSearch)
* 5512 - PostgreSQL
* 7512 - Redoc API documentation
* 8512 - Auth server

## Architecture

### Rust
* **src/main.rs** - main server loop, runs actions according to received request
* **src/db.rs** - database connection
* **src/model.rs** - structs for encoding/decoding JSON and database rows
* **src/schema.rs** - database schema
* **src/actions.rs** - server actions
* **Cargo.toml** - build config

### Diesel
* migrations/ - initial data imported to database

For local initial configuration run:
`diesel setup && diesel migration run --database-url=<db_url>`
If you need to redo the import run
`diesel migration redo --database-url=<db_url>`

Note that you might need to install diesel cli tools first. 
