# Install


This project uses docker-compose. Install Docker if you haven't.

## Devevelopment

Install and startup all services.

`docker-compose -f docker-compose.dev.yml up --build`

Populate the data from disk:

`cargo run --bin update -- --root-folder place-on-disk`

## Production

`docker-compose up -d`

## TODO
- Make update run in Docker, periodically
- Update rapid before running updater


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