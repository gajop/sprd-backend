# sprd-backend

Provides the fast metadata query API for SpringRTS Rapid protocol.

System architecture:

![System architecture](./docs/Backend%20System.png)


# Install


This project uses docker-compose. Install Docker if you haven't.

## Development

(TODO: Describe instructions on installing Postgres. But rather than that, do server compilation inside a Docker container?)

## Install Postgres

We need to install Postgres so Diesel can build and link. This Postgres won't actually be used for connectivity, only for compilation.

### Windows

Install latest Postgres from https://www.postgresql.org/download/windows/
`setx PQ_LIB_DIR "C:\Program Files\PostgreSQL\13\lib"`

### Linux

Install Postgres (Ubuntu instructions given below):

```bash
sudo apt update
sudo apt install postgresql postgresql-contrib libpq-dev
```

## Build

Build the server.

`cargo build`

Install and startup all services.

`docker-compose -f docker-compose.yml -f docker-compose.dev.yml up --build`

Populate the data from disk:

`cargo run --bin update -- --root-folder place-on-disk`

## Production

```sh
docker-compose -f docker-compose.yml -f docker-compose.prod.yml pull
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up
```

PS: Don't run with the --build

### AWS

See this gist for how to setup docker-compose on AWS: https://gist.github.com/npearce/6f3c7826c7499587f00957fee62f8ee9

## TODO
- Store .env on GitHub
- Explain sprd binary
- use sprd as a library instead of copy pasting parts


<!-- OLD:

## Setup DB

```
sudo apt update
sudo apt install postgresql postgresql-contrib libpq-dev
```

```
sudo -u postgres createuser --interactive
sudo -u postgres createdb spm
``` -->