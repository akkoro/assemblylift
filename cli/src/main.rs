extern crate serde_json;

use clap::{crate_version, Arg, App};
use crate::commands::init::init;
use crate::commands::compile::compile;
use std::collections::HashMap;
use crate::commands::CommandFn;

mod commands;
mod projectfs;
mod templates;

fn main() {
    let app = App::new("asml")
        .version(crate_version!())
        .subcommand(
            App::new("init")
                .arg(
                    Arg::with_name("language")
                        .short("l")
                        .long("lang")
                        .default_value("rust")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("project_name")
                        .short("n")
                        .long("name")
                        .required(true)
                        .takes_value(true)
                )
                // TODO this is going to need an argument to specify the backend (ie aws-lambda, azure, etc)
        )
        .subcommand(
            App::new("compile")
        );
    let matches = app.get_matches();

    let mut command_map = HashMap::<&str, CommandFn>::new();
    command_map.insert("init", init);
    command_map.insert("compile", compile);

    match matches.subcommand() {
        (name, matches) => command_map[name](matches)
    }
}
