# RShorty

A simple keyword to url mapper service similar to
[url-mapper](https://github.com/dhruvasagar/url-mapper) written in Rust
programming language

# Setup

## Pre-requisites

* [Rust](https://rust-lang.org)
* `sqlx-cli` to create the database (sqlite) using `sqlx database create`
  assuming env `DATABASE_URL` is set
* `Sqlite` (optional)
* Docker (optional), easier to build & run the project in a container

## Compilation

```sh
$ cargo build --release
```
## Running

### Using Cargo
```sh
$ cargo run
```
### Using Docker
```sh
$ docker build -t rshorty .
$ docker run --rm -it rshorty
```
