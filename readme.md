# Cargo Rocket Docker

Automate tool sub-command to create a docker image for Rocket Project and build/publish them using Alpine OS.

## References 

- https://crates.io/crates/cargo-rocket-docker
- https://github.com/Gary-Ascuy/cargo-rocket-docker
- https://github.com/Gary-Ascuy/rust-rest-api
- https://hub.docker.com/repository/docker/garyascuy/cargo-rocket-example

## Development Setup

#### Software Requirements 

- Rust Nightly Version
- Docker Latest

#### Install into Environment

One-Time Install from Repo
```sh
$ cargo install --path . 
```

Development Mode ( Execute Install after each update)
```sh
$ cargo watch -x "install --path ."
```

Check
```sh
$ cargo docker --help
```

#### Manual Tests ( WIP: Unit Tests )

```sh
# Clone Test Repository
$ git clone https://github.com/Gary-Ascuy/rust-rest-api.git
$ cd rust-rest-api

# Basic Commands
$ cargo docker --build
$ cargo docker --push

# All-In-One Commands
$ cargo docker --build --push
$ cargo docker --all

# Eject Dockerfile && .dockerignore files
$ cargo docker --eject
```

## Docker Image 

```sh
garyascuy/cargo-rocket-example:1.0.0
OS/ARCH: linux/amd64
SIZE: 4.31 MB
LAST PUSHED: 12 minutes ago by garyascuy
```

## About

Created by [Gary Ascuy][garyascuygithub] and Follow me in [LinkedIn][garyascuylinkedin] or [GitHub][garyascuygithub] if you want :P.

[garyascuygithub]: https://github.com/gary-ascuy
[garyascuylinkedin]: https://www.linkedin.com/in/gary-ascuy-6619bbb9/
