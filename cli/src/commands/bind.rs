use clap::ArgMatches;

use crate::terraform;

pub fn command(matches: Option<&ArgMatches>) {
    let _matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for bind command"),
    };

    terraform::commands::init();
    terraform::commands::apply();
}
