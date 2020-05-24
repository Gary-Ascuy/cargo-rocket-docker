// use std::path::Path;
// use std::io::Write;
// use std::fs::File;

use serde_json::json;
mod template;

fn main() {
    let hbs = &template::register_templates();

    let data = &json!({
        "name": "gary",
        "maintainer": "Gory Ascuy <gary.ascuy@gmail.com>",
        "packages": {
            "build": "",
            "image": "", // "imagemagick"
        },
    });
    let dockerfile = template::dockerfile(hbs, data);
    let dockerignore = template::dockerignore(hbs, data);

    // File Create
    // let path = Path::new("./Dockerfile");
    // let mut file = File::create(&path).expect("There is not possible update files");
    // file.write_all(template.as_bytes()).expect("Gary");

    println!("{}", dockerfile);
    println!("{}", dockerignore);
}
