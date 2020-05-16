use clap::{crate_version, Arg, App, SubCommand};
use crate::commands::init::init;
use std::collections::HashMap;
use crate::commands::CommandFn;

mod commands;

fn main() {
    let app = App::new("asml")
        .version(crate_version!())
        .subcommand(
            App::new("init")
                .arg(
                    Arg::with_name("language")
                        .short("l")
                        .default_value("rust")
                        .takes_value(true)
                )
        );
    let matches = app.get_matches();

    let mut command_map = HashMap::<&str, CommandFn>::new();
    command_map.insert("init", init);

    match matches.subcommand() {
        (name, matches) => command_map[name](matches),
        _ => {}
    }
}
