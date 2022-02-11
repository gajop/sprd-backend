# sprd-backend

Provides the fast metadata query API for SpringRTS Rapid protocol.

System architecture:

![System architecture](./docs/Backend%20System.png)


# Install


This project uses docker-compose. Install Docker if you haven't.

## Development


Install and startup all services.

`docker-compose -f docker-compose.yml -f docker-compose.dev.yml up --build`

Populate the data from disk:

`cargo run --bin update -- --root-folder place-on-disk`

## Production

`docker-compose -f docker-compose.yml -f docker-compose.prod.yml up --build`

## TODO
- Setup production Docker compose
- Store .env on GitHub
- Explain sprd binary
- use sprd as a library instead of copy pasting parts


<!-- OLD:

## System (SQLite)

`sudo apt install sqlite3 libsqlite3-0 libsqlite3-dev`

## Project

`cargo build`

# Dev setup

`cargo install diesel_cli --no-default-features --features sqlite`

## Setup DB

```
sudo apt update
sudo apt install postgresql postgresql-contrib libpq-dev
```

```
sudo -u postgres createuser --interactive
sudo -u postgres createdb spm
``` -->