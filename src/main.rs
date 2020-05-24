// use serde_json::json;
mod template;
mod config;

fn main() {
    let value: config::Config = config::parse();

    // let data = &json!({
    //     "name": "server",
    //     "maintainer": "Gary Ascuy <gary.ascuy@gmail.com>",
    //     "packages": { // Alpine packages to install in develop & final image
    //         "build": "",
    //         "image": "", // "imagemagick"
    //     },
    //     "docker:tags": [{ // docker image tags
    //         "name": "@garyascuy/server",
    //         "version": "1.0.0",
    //     }, {
    //         "name": "@garyascuy/server",
    //         "version": "latest",
    //     }]
    // });

    template::save("./example", &value);
}
