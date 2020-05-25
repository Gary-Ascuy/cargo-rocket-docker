use clap::{App, Arg, SubCommand};
use colored::*;

mod template;
mod config;

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

    // let all = options.is_present("all");
    // let build = options.is_present("build");
    // let push = options.is_present("push");

    let eject = options.is_present("eject");
    let value = config::parse();

    match eject {
        true => {
            println!("Eject Command Execution - Cargo Docker");
            println!("Project Path: {}", env!("CARGO_MANIFEST_DIR"));
            
            template::save(".", &value);
            println!("{}", "Task completed successfully, files:".bright_green());
            println!("    - {}", "./Dockerfile".bright_green());
            println!("    - {}", "./.dockerignore".bright_green());
            std::process::exit(0);
        },
        false => {
            println!("{}", "WIP - Under Implementation".bright_red());
            println!("{}", "Gary Ascuy <gary.ascuy@gmail.com>".bright_red());
            std::process::exit(1);
        },
    }
}
