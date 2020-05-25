# Cargo Rocket Docker

Automate tool sub-command to create a docker image for Rocket Project and build/publish them using Alpine OS.

## Install 

```sh
$ cargo install cargo-rocket-docker
```

To upgrade:
```sh
$ cargo install --force cargo-rocket-docker
```

## Usage

Add following section into `Cargo.toml` file
```toml
[docker]
name = "app-bin-name"
version = "0.1.0"
maintainer = "Team Name <mail.support@company.com>"
temp_folder = "./.tmp_docker"
tag = "garyascuy/cargo-rocket-example"
```

Build Docker Images
```sh
$ cargo docker --build
```

Push Docker Images
```sh
$ cargo docker --push
```

All-In-One Command? Type following command:
```sh
$ cargo docker --build --push --keep-temporary-files
```

All-In-One Command still long? Here more alternatives:
```sh
$ cargo docker --all
$ cargo docker -a
$ cargo docker -bpk # [B]uild, [P]ush, [K] Keep Docker Files
```

Do you need some custom? Create files and maintaing by yourself ಠ_ಠ:
```sh
$ cargo docker --eject
```

There's a lot more you can do! Here's a copy of the help:
```sh
USAGE:
    cargo docker [FLAGS]

FLAGS:
    -a, --all                     build and publish docker images
    -b, --build                   build docker images
    -e, --eject                   eject Dockerfile and .dockerignofe files
    -h, --help                    Prints help information
    -k, --keep-temporary-files    Keep temporary files after build execution
    -p, --push                    publish docker images
    -V, --version                 Prints version information
```

## Config Properties

Here Full Example && Custom Tags && Alpine Packages
```toml
[docker]
name = "app-bin-name"
version = "0.1.0"
maintainer = "Team Name <mail.support@company.com>"
temp_folder = "./.tmp_docker"
tag = "garyascuy/cargo-rocket-example"
custom_tags = [
    "registry.gitlab.com/garyascuy:latest",
    "registry.gitlab.com/garyascuy:1.0.0",
]

[docker.packages]
build = "acf-openssl"
image = "imagemagick second-pkg other-pkg"
```

Descriptions
```toml
# Cargo Docker Settings
[docker]
name = "app-bin-name" # docker will copy from target/release/{app-bin-name}
version = "1.0.0"     # Docker image version
maintainer = "Team Name <mail@company.com>" # Responsible for image
tag = "account/back" # Docker tag base, it will create account/back:{version} and account/back:latest
custom_tags = [ # docker build will use these spesific tags to create the images and publish
    "garyascuy/server:1.0.0",
    "garyascuy/server:latest",
]

# Optional install extra packages in Alpine OS
[docker.packages]
build = "acf-openssl" # apk add acf-openssl in build image, development dependencies
image = "imagemagick second-pkg other-pkg" # add packages in final image, production dependencies
```

## About

Created by [Gary Ascuy][garyascuygithub] and Follow me in [LinkedIn][garyascuylinkedin] or [GitHub][garyascuygithub] if you want :P.

[garyascuygithub]: https://github.com/gary-ascuy
[garyascuylinkedin]: https://www.linkedin.com/in/gary-ascuy-6619bbb9/
