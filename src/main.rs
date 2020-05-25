use clap::{App, Arg, SubCommand};
use colored::*;

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
        )
        .get_matches();

    let options = opts.subcommand_matches("docker")
        .expect("cargo-rocket-docker must be used as a subcommand");

    let all = options.is_present("all");
    let build = all || options.is_present("build");
    let push = all || options.is_present("push");
    let eject = options.is_present("eject");
    let value = config::parse();

    if eject {
        print_into(&"Eject".bright_green());
        
        template::save(".", &value);
        println!("{}", "Task completed successfully, files:".bright_green());
        println!("    - {}", "./Dockerfile".bright_green());
        println!("    - {}", "./.dockerignore".bright_green());
        std::process::exit(0);
    }

    let mut result = 1;

    if build {
        print_into(&"Build".bright_green());
        result = 0;
    }

    if push {
        print_into(&"Push".bright_green());
        result = 0;
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
            std::process::exit(result);
        }
    }
}
