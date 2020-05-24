// use std::path::Path;
// use std::io::Write;
// use std::fs::File;

use serde_json::json;
mod template;

fn main() {
    let data = &json!({
        "name": "server",
        "maintainer": "Gary Ascuy <gary.ascuy@gmail.com>",
        "packages": {
            "build": "",
            "image": "", // "imagemagick"
        },
    });

    template::save("./example", data);
}
