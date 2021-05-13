use clap::ArgMatches;

use crate::terraform;

pub fn command(_matches: Option<&ArgMatches>) {
    terraform::commands::destroy();
}
