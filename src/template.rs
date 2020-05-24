use std::path::{Path, PathBuf};
use std::io::Write;
use std::fs::File;

use handlebars::{Handlebars, RenderContext, Helper, Context, JsonRender, HelperResult, Output};
use serde::ser::Serialize;

fn choose_helper(h: &Helper, _: &Handlebars, _: &Context, _rc: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    let default = h.param(1).unwrap();
    let value = h.param(0).unwrap_or(default);
    match value.value().render().as_mut_str().trim().is_empty() {
        true => out.write(default.value().render().as_ref())?,
        false => out.write(value.value().render().as_ref())?,
    }
    Ok(())
}

pub fn register_templates() -> Handlebars<'static> {
    let mut hbs = Handlebars::new();
    hbs.register_template_file("Dockerfile", "./templates/Dockerfile.hbs").expect("Could not read ./template/Dockerfile.hbs file.");
    hbs.register_template_file(".dockerignore", "./templates/.dockerignore.hbs").expect("Could not read ./template/.dockerignore.hbs file.");

    hbs.register_helper("choose", Box::new(choose_helper));
    
    hbs
}

pub fn dockerfile<T>(hbs: &Handlebars<'static>, data: &T) -> String
where
    T: Serialize
{
    hbs.render("Dockerfile", data).expect("Could not render docker tempalte")
}

pub fn dockerignore<T>(hbs: &Handlebars<'static>, data: &T) -> String
where
    T: Serialize
{
    hbs.render(".dockerignore", data).expect("There is a problem with template")
}

pub fn save_file(content: &str, path: &PathBuf) {
    let mut file = File::create(&path).expect("There is not possible update files");
    file.write_all(content.as_bytes()).expect("Could not save data into file");
}

pub fn save<T>(project_path: &str, data: &T)
where
    T: Serialize
{
    let hbs = &register_templates();
    let path = Path::new(project_path).canonicalize().expect("Could not found project path.");
    let dir = Path::new(path.as_os_str());

    save_file(&dockerfile(hbs, data), &dir.join("Dockerfile"));
    save_file(&dockerignore(hbs, data), &dir.join(".dockerignore"));
}
