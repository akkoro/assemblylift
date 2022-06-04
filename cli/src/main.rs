extern crate serde_json;

use clap::{App, AppSettings, Arg, crate_version};

use crate::commands::{bind, burn, cast, init, make, pack, push, user};

mod archive;
mod commands;
mod projectfs;
mod providers;
mod templates;
mod terraform;
mod tools;
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
            .arg(
                Arg::with_name("language")
                    .short("l")
                    .takes_value(true)
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
        );
    let matches = app.setting(AppSettings::ArgRequiredElseHelp).get_matches();

    match matches.subcommand() {
        ("init", matches) => init::command(matches),
        ("cast", matches) => cast::command(matches),
        ("bind", matches) => bind::command(matches),
        ("burn", matches) => burn::command(matches),
        ("make", matches) => make::command(matches),
        ("pack", matches) => pack::command(matches),
        ("push", matches) => push::command(matches),
        ("user", matches) => user::command(matches),
        (cmd, _) => println!("Invalid subcommand `{}`. Try `asml help` for options.", cmd),
    }
}
