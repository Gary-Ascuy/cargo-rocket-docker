# Cargo Rocket Docker - Work In Progress (IMPORTANT)

Sub-command to create a docker image for Rocket Project and build using Alpine OS.

### Features 

Creates Dockerfile & .dockerignore files into a rocket project.
```sh
$ cargo docker --eject
```

#### Add following settings in your `Cargo.toml`

Basic Configuration
```toml
[docker]
name = "package-name"
version = "1.0.0"
maintainer = "Team Name <mail@company.com>"
tag = "garyascuy/package-name"
```

Alternative with Alpine PKG Dependencies for development and production
```toml
[docker]
name = "package-name"
version = "1.0.0"
maintainer = "Team Name <mail@company.com>" 

[docker.packages]
build = "acf-openssl"
image = "imagemagick second-pkg other-pkg"
```

Descriptions
```toml
[docker]
name = "package-name" # docker will copy from target/release/{package-name}
version = "1.0.0"     # Docker image version
maintainer = "Team Name <mail@company.com>" 
tag = "account/back" # Docker tag base, it will create account/back:{version} and account/back:latest
tags = [ # docker build will use these spesific tags to create the images and publish
    "garyascuy/server:1.0.0",
    "garyascuy/server:latest",
]

# Optional install extra packages in Alpine OS
[docker.packages]
build = "acf-openssl" # apk add acf-openssl in build image, development dependencies
image = "imagemagick second-pkg other-pkg" # add packages in final image, production dependencies
```

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
