use clap::{crate_version, Arg, App, SubCommand};

fn main() {
    let app = App::new("asml")
        .version(crate_version!())
        .subcommand(
            App::new("init")
        );
    let matches = app.get_matches();

    match matches.subcommand() {
        ("init", init_matches) => {}
        _ => {}
    }
}
