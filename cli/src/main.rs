extern crate serde_json;

use std::collections::HashMap;

use clap::{crate_version, App, Arg};

use crate::commands::CommandFn;
use crate::commands::{bind, burn, cast, init};

mod bom;
mod artifact;
mod commands;
mod projectfs;
mod templates;
mod terraform;

fn main() {
    let app = App::new("asml")
        .version(crate_version!())
        .subcommand(
            App::new("init")
                .about("Initialize a basic AssemblyLift application")
                .arg(
                    Arg::with_name("language")
                        .short("l")
                        .long("lang")
                        .default_value("rust")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("project_name")
                        .short("n")
                        .long("name")
                        .required(true)
                        .takes_value(true),
                ), // TODO this is going to need an argument to specify the backend (ie aws-lambda, azure, etc)
        )
        .subcommand(App::new("cast").about("Build the AssemblyLift application"))
        .subcommand(
            App::new("bind")
                .about("Bind the application to the cloud backend")
                .alias("sync"),
        )
        .subcommand(
            App::new("burn")
                .about("Destroy all infrastructure created by 'bind'")
                .after_help("Equivalent to 'terraform destroy'"),
        );
    let matches = app.get_matches();

    let mut command_map = HashMap::<&str, CommandFn>::new();
    command_map.insert("init", init::command);
    command_map.insert("cast", cast::command);
    command_map.insert("bind", bind::command);
    command_map.insert("burn", burn::command);

    match matches.subcommand() {
        (name, matches) => command_map[name](matches),
    }
}
