use clap::{App, Arg, SubCommand};
use colored::*;
use std::fs;
use std::path::Path;
use std::process::Command;

mod template;
mod config;

fn print_into(command: &ColoredString) {
    let message = format!("{} Command Execution - Cargo Docker @ {}", command, "Gary Ascuy".bright_blue());
    println!("{}", message);
    println!("Project: {}", env!("CARGO_MANIFEST_DIR"));
    println!("");
}

fn get_all_tags(config: &mut config::Config) -> Vec<String> {
    let mut all_tags = Vec::new();
    let name = &config.package.name;
    let version = &config.package.version;

    if config.docker.is_some() {
        let docker = config.docker.clone().unwrap();
        let tag = docker.tag;
        let docker_version = docker.version;
        let custom_tags = docker.custom_tags;

        if tag.is_some() {
            let value = tag.unwrap();
            all_tags.push(format!("{}:{}", value, docker_version.unwrap_or(version.clone())));
            all_tags.push(format!("{}:{}", value, "latest"));
        }

        if custom_tags.is_some() {
            for custom_tag in custom_tags.unwrap() {
                all_tags.push(custom_tag);
            }
        }
    } else {
        all_tags.push(format!("{}:{}", name, version));
        all_tags.push(format!("{}:{}", name, "latest"));
    }

    return all_tags;
}

fn main() {
    let opts = App::new("cargo")
        .bin_name("cargo")
        .subcommand(
            SubCommand::with_name("docker")
                .version(env!("CARGO_PKG_VERSION"))
                .about(env!("CARGO_PKG_DESCRIPTION"))
                .author("Gary Ascuy <gary.ascuy@gmail.com>")
                .arg(
                    Arg::with_name("build")
                        .help("build docker images")
                        .short("b")
                        .long("build"),
                )
                .arg(
                    Arg::with_name("push")
                        .help("publish docker images")
                        .short("p")
                        .long("push"),
                )
                .arg(
                    Arg::with_name("eject")
                        .help("eject Dockerfile and .dockerignofe files")
                        .short("e")
                        .long("eject"),
                )
                .arg(
                    Arg::with_name("all")
                        .help("build and publish docker images")
                        .short("a")
                        .long("all"),
                )
                .arg(
                    Arg::with_name("keep-temporary-files")
                        .help("Keep temporary files after build execution")
                        .short("k")
                        .long("keep-temporary-files"),
                )
        )
        .get_matches();

    let options = opts.subcommand_matches("docker")
        .expect("cargo-rocket-docker must be used as a subcommand");

    let all = options.is_present("all");
    let build = all || options.is_present("build");
    let push = all || options.is_present("push");
    let eject = options.is_present("eject");
    let keep_files = options.is_present("keep-temporary-files");

    let value = config::parse();

    if eject {
        print_into(&"Eject".bright_green());
        
        template::save(".", &value);
        println!("{}", "Task completed successfully, files:".bright_green());
        println!("    - {}", "./Dockerfile".bright_green());
        println!("    - {}", "./.dockerignore".bright_green());
        println!("");
        std::process::exit(0);
    }

    let mut result = 1;

    if build {
        print_into(&"Build".bright_green());

        let temp_conifg = value.clone();
        let tmp = match temp_conifg.docker.is_some() {
            true => temp_conifg.docker.unwrap().temp_folder.unwrap_or(String::from("./.tmp_cargo_docker")),
            false => String::from("./.tmp_cargo_docker"),
        };

        fs::create_dir_all(tmp.as_str()).expect("Could not create temporary directory for Docker");
        template::save(tmp.as_str(), &value.clone());
        let dockerfile = Path::new(tmp.as_str()).join("./Dockerfile").canonicalize().expect("Could not found Dockerfile in temporary directory");

        let mut command = Command::new("docker");
        command.arg("build");

        let mut all_tags_config = value.clone();
        let all_tags = get_all_tags(&mut all_tags_config);
        for tag in all_tags.clone() {
            command.arg("-t").arg(tag);
        }

        command.arg("--file").arg(dockerfile.as_os_str());
        command.arg(".");

        println!("{}", "Docker Command".bright_green());

        println!("$ docker build \\\n    -t \"{}\" \\\n    --file \"{}\" \".\"", all_tags.clone().join("\" \\\n    -t \""), dockerfile.display());
        println!("");
        
        println!("{}", "Command Execution".bright_green());
        let status = command.status().expect("failed to execute process");
        
        if !keep_files {
            fs::remove_dir_all(tmp).expect("Could not remove temporal directory for Docker");
        }
        
        println!("");
        result = status.code().expect("Could not load status from Docker execution");
        
        if result != 0 {
            println!("{}", "Could not run docker build command".bright_red());
            println!("");
            std::process::exit(result)
        }
    }

    if push {
        print_into(&"Push".bright_green());

        let mut all_tags_config = value.clone();
        let all_tags = get_all_tags(&mut all_tags_config);

        let commands = all_tags.join("\"\n$ docker push \"");
        println!("{}", "Docker Commands".bright_green());
        println!("$ docker push \"{}\"", commands);
        println!("");

        println!("{}", "Commands Execution".bright_green());
        for tag in all_tags {
            let command =  Command::new("docker").arg("push").arg(tag).status().expect("Could not docker push");
            let exec_code = command.code().expect("Error");
            if exec_code != 0 {
                std::process::exit(exec_code);
            }
        }

        println!("");
        result = 0;

        if result != 0 {
            println!("{}", "Could not run docker push commands".bright_red());
            println!("");
            std::process::exit(result)
        }
    }

    match result == 0 {
        true => std::process::exit(result),
        false => {
            println!("{} {}", "¯\\_(ツ)_/¯".blue(), "so I dont know what do you want to execute, please help me!".bright_red());

            println!("");
            println!("Cargo Docker Examples:");
            println!("    - {}", "cargo docker --build");
            println!("    - {}", "cargo docker --push");
            println!("    - {}", "cargo docker --all");
            println!("    - {}", "cargo docker --eject");
            println!("");

            println!("{}", "Cargo Docker @ Gary Ascuy <gary.ascuy@gmail.com>".bright_blue());
            println!("");
            std::process::exit(result);
        }
    }
}
