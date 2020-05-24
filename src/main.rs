extern crate handlebars;
// #[macro_use]
extern crate serde_json;

use handlebars::Handlebars;
// use serde_json::value::{ Map };

fn main() {
    let mut hbs = Handlebars::new();
    hbs.register_template_file("Dockerfile", "./templates/Dockerfile.hbs").expect("Could not read ./template/Dockerfile.hbs file.");
    hbs.register_template_file("Dockerfile", "./templates/.dockerignore.hbs").expect("Could not read ./template/.dockerignore.hbs file.");

    // println!(
    //     "{}",
    //     hbs.render_template("Hello {{name}}", &json!({"name": "foo"}))
    // );

    // let data = &json!({"name": "foo"});
    
    
    // println!("Hello, world! {}", hbs.render("Dockerfile", &json!({"name": "foo"})));
    // println!("Hello, world! {}", &data);
}
