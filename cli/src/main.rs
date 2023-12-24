extern crate serde_json;

use clap::{crate_version, App, AppSettings, Arg};

use crate::commands::{bind, burn, cast, host, init, make, nuke, pack, push, r#move, user};

mod archive;
mod commands;
mod templates;

fn main() {
    let app = App::new("asml")
        .version(crate_version!())
        .subcommand(
            App::new("init")
                .about("Initialize a new AssemblyLift application")
                // .arg(
                //     Arg::with_name("language")
                //         .short("l")
                //         .long("lang")
                //         .default_value("rust")
                //         .takes_value(true),
                // )
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
            .arg(
                Arg::with_name("language")
                    .short("l")
                    .takes_value(true)
            )
        )
        .subcommand(App::new("move")
            .about("Rename a service or function, or move a function between services")
            .after_help("RESOURCE SYNTAX:\n    asml move service <service-name> <new-name>\n    asml move function <service-name>.<function-name> <new-service>.<new-function>")
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
                .about("Delete a service or function")
                .after_help("RESOURCE SYNTAX:\n    asml burn service <service-name>\n    asml burn function <service-name>.<function-name>")
                .arg(
                    Arg::with_name("resource")
                        .multiple(true)
                        .required(true)
                ),
        )
        .subcommand(
            App::new("nuke")
                .about("Destroy/de-provision ALL deployed infrastructure"),
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
                        ),
                ),
        )
        .subcommand(
            App::new("push")
                .about("Push artifacts to a registry")
                .subcommand(
                    App::new("iomod")
                        .about("Publish an IOmod to the public registry")
                        .arg(
                            Arg::with_name("auth-header")
                                .long("auth-header")
                                .required(true) // temporarily true until user login is finished
                                .takes_value(true)
                        )
                        .arg(
                            Arg::with_name("package")
                                .long("package")
                                .required(true)
                                .takes_value(true)
                        )
                        .arg(
                            Arg::with_name("coords")
                                .long("coords")
                                .required(true)
                                .takes_value(true)
                        ),
                ),
        )
        .subcommand(
            App::new("user")
                .about("User authentication & information")
                .subcommand(
                    App::new("login")
                        .about("Login to the IOmod registry")
                ),
        )
        .subcommand(
            App::new("host")
                .about("Spawn a local development server")
        );
    let matches = app.setting(AppSettings::ArgRequiredElseHelp).get_matches();

    match matches.subcommand() {
        ("init", matches) => init::command(matches),
        ("cast", matches) => cast::command(matches),
        ("bind", matches) => bind::command(matches),
        ("burn", matches) => burn::command(matches),
        ("make", matches) => make::command(matches),
        ("move", matches) => r#move::command(matches),
        ("nuke", matches) => nuke::command(matches),
        ("pack", matches) => pack::command(matches),
        ("push", matches) => push::command(matches),
        ("user", matches) => user::command(matches),
        ("host", matches) => host::command(matches),
        (cmd, _) => println!("Invalid subcommand `{}`. Try `asml help` for options.", cmd),
    }
}
