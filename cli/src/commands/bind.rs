use clap::ArgMatches;

use crate::terraform;

pub fn command(matches: Option<&ArgMatches>) {
    let matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for bind command")
    };

    terraform::run_terraform_init();
    terraform::run_terraform_apply();
}