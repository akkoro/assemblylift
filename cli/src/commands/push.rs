use clap::ArgMatches;

pub fn command(matches: Option<&ArgMatches>) {
    let matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for cast command"),
    };
    
    match matches.subcommand() {
        _ => println!("{}", "missing subcommand. try `asml pack help` for options."),
    }
}
