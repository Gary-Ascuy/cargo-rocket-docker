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
        let tmp = "./some/dir";
        fs::create_dir_all(tmp).expect("Could not create temporary directory for Docker");
        template::save(tmp, &value.clone());
        let dockerfile = Path::new(tmp).join("./Dockerfile").canonicalize().expect("Could not found Dockerfile in temporary directory");

        let mut command = Command::new("docker");
        command.arg("build");

        // match value.docker.is_some() {
        //     true => 
        // }

        let mut check = value.clone();
        if check.docker.is_some() && check.docker.unwrap().tag.is_some() {
            let docker = value.clone();
            let tag = format!("{}", docker.docker.unwrap().tag.unwrap());
            command.arg("-t");
            command.arg(format!("{}:{}", tag, value.package.version.as_str()));

            command.arg("-t");
            command.arg(format!("{}:{}", tag, "latest"));
        }

        check = value.clone();
        if check.docker.is_some() && check.docker.unwrap().custom_tags.is_some() {
            for tag in value.clone().docker.unwrap().custom_tags.unwrap() {
                command.arg("-t");
                command.arg(tag);
            }
        }

        command.arg("--file");
        command.arg(dockerfile.as_os_str());
        command.arg(".");

        println!("{}", "Docker Command".bright_green());
        println!("  - {:?}", command);
        println!("");

        println!("********************    DOCKER    ********************");
        
        let status = command.status().expect("failed to execute process");
        
        if !keep_files {
            fs::remove_dir_all(tmp).expect("Could not remove temporal directory for Docker");
        }
        
        println!("******************************************************");
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
        
        let mut check = value.clone();
        let mut all_tags = vec![];
        if check.docker.is_some() && check.docker.unwrap().tag.is_some() {
            let docker = value.clone();
            let tag = format!("{}", docker.docker.unwrap().tag.unwrap());
            let tag_version = format!("{}:{}", tag, value.package.version.as_str());
            let tag_latest = format!("{}:{}", tag, "latest");
            all_tags.push(tag_version);
            all_tags.push(tag_latest);

            // Command::new("docker").arg("push").arg(tag_version).status().expect("Push");
            // Command::new("docker").arg("push").arg(tag_latest).status().expect("Push");
        }

        check = value.clone();
        if check.docker.is_some() && check.docker.unwrap().custom_tags.is_some() {
            for tag in value.clone().docker.unwrap().custom_tags.unwrap() {
                all_tags.push(tag);
                // Command::new("docker").arg("push").arg(tag).status().expect("Push");
            }
        }

        println!("{}", "Docker Commands".bright_green());
        println!("  - docker push {}", all_tags.join("\n  - docker push "));
        println!("");

        for tag in all_tags {
            let code = Command::new("docker").arg("push").arg(tag).status().expect("Push").code().expect("Error");
            if code != 0 {
                std::process::exit(code);
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
