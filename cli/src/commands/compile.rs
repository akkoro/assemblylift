use clap::ArgMatches;

pub fn compile(matches: Option<&ArgMatches>) {
    let matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for compile")
    };

    
}
