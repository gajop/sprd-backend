# Install

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
```
