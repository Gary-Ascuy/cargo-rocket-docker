# Cargo Rocket Docker - Work In Progress (IMPORTANT)

WIP: Sub command to create a docker image for Rocket Project.

### IDEA

Creates a docker image based on alpine for project, and it allow you put tags tofor images, if you have operation system dependencies you can espesify into configuration.

```sh
$ cargo docker build

# Variant for all flow
$ cargo docker build --push 
```

Publish all images into the Docker repository.
```sh
$ cargo docker push 
```

Creates Dockerfile and ingore file to custumize acording to your requirement.
```sh
$ cargo docker exject 
```

Custom Configuration into `Cargo.toml`
```toml
[docker]
name = 'server'
version = '1.6.6'
maintainer = 'fromfile<file@from.com>'
tag = "garyascuy/server"
tags = [
    "garyascuy/server:1.0.0",
    "garyascuy/server:latest",
]

[docker.packages]
build = "imagemagick"
image = "gary"
```
