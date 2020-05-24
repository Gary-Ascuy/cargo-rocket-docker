use handlebars::Handlebars;
use serde;

pub fn register_templates() -> handlebars::Handlebars<'static> {
    let mut hbs = Handlebars::new();
    hbs.register_template_file("Dockerfile", "./templates/Dockerfile.hbs").expect("Could not read ./template/Dockerfile.hbs file.");
    hbs.register_template_file(".dockerignore", "./templates/.dockerignore.hbs").expect("Could not read ./template/.dockerignore.hbs file.");
    hbs
}

pub fn dockerfile<T>(hbs: &handlebars::Handlebars<'static>, data: &T) -> String
where
    T: serde::ser::Serialize
{
    hbs.render("Dockerfile", data).expect("Could not render docker tempalte")
}

pub fn dockerignore<T>(hbs: &handlebars::Handlebars<'static>, data: &T) -> String
where
    T: serde::ser::Serialize
{
    hbs.render(".dockerignore", data).expect("There is a problem with template")
}
