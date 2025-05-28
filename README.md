# Axum, SQLx and PostgreSQL

Introduces Axum, SQLx, and PostgreSQL to build scalable, type-safe, and production-ready REST APIs.

## Features

- **Axum**: Modern async web framework for Rust
- **SQLx**: SQL crate that simplifies database interactions
- **PostgreSQL**: Robust relational database
- **Docker**: Containerized deployment (optional)

## Running the Project

### 1. Prerequisites

Make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) or Docker
- [PostgreSQL](https://www.postgresql.org/download/)
- [`sqlx-cli`](https://crates.io/crates/sqlx-cli) (for running migrations)

Install SQLx CLI:

```shell
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

### 2. Environment Variables

Copy the example environment file to _.env_ and adjust the values as needed.

### 3. Database Setup

1. Create the database (if it doesn't exist)

This will create the database specified in the `DATABASE_URL` from your _.env_ file.

```shell
sqlx database create
```

2. Run database migrations

```shell
sqlx migrate run
```

### 4. Run the Server

#### Option A: Run locally with Cargo

```shell
cargo run
```

#### Option B: Run with Docker

1. Build the image

```shell
docker build -t axum-api .
```

2. Run the container

```shell
docker run -d --name axum-api-container -p 3000:3000 --env-file .env axum-api
```

> [!IMPORTANT]
> This only runs the Axum server. You still need a running PostgreSQL instance accessible via the DATABASE_URL defined in your _.env_ file.

Common Docker Commands:

```shell
# Stop the container
docker stop axum-api-container

# Start the container
docker start axum-api-container

# View logs
docker logs axum-api-container

# Remove the container
docker rm -f axum-api-container

# Remove the image
docker rmi axum-api
```
