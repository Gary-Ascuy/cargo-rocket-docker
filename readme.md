# Cargo Rocket Docker

Automate tool sub-command to create a docker image for Rocket Project and build/publish them using Alpine OS.

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

## About

Created by [Gary Ascuy][garyascuygithub] and Follow me in [LinkedIn][garyascuylinkedin] or [GitHub][garyascuygithub] if you want :P.

[garyascuygithub]: https://github.com/gary-ascuy
[garyascuylinkedin]: https://www.linkedin.com/in/gary-ascuy-6619bbb9/
