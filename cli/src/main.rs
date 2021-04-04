extern crate serde_json;

use std::collections::HashMap;

use clap::{crate_version, App, Arg};

use crate::commands::CommandFn;
use crate::commands::{bind, burn, cast, init, make, pack};

mod archive;
mod commands;
mod docker;
mod projectfs;
mod providers;
mod templates;
mod terraform;
mod transpiler;

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
                ),
        )
        .subcommand(App::new("make")
            .about("Make a new service or function")
            .after_help("RESOURCE SYNTAX:\n    asml make service <service-name>\n    asml make function <service-name>.<function-name>")
            .arg(
                Arg::with_name("resource")
                    .multiple(true)
                    .required(true)
            )
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
        )
        .subcommand(
            App::new("pack")
                .about("Pack artifacts for publishing")
                .subcommand(
                    App::new("iomod")
                        .about("Pack an IOmod for publishing")
                        .arg(
                            Arg::with_name("out")
                                .short("o")
                                .required(true)
                                .takes_value(true)
                        )
                ),
        );
    let matches = app.get_matches();

    // TODO why did I do this with a map? just match strings below in the match clause
    let mut command_map = HashMap::<&str, CommandFn>::new();
    command_map.insert("init", init::command);
    command_map.insert("cast", cast::command);
    command_map.insert("bind", bind::command);
    command_map.insert("burn", burn::command);
    command_map.insert("make", make::command);
    command_map.insert("pack", pack::command);

    match matches.subcommand() {
        (name, matches) => command_map[name](matches),
    }
}
