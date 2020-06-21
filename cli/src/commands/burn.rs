use clap::ArgMatches;

use crate::terraform;

pub fn command(matches: Option<&ArgMatches>) {
    terraform::run_terraform_destroy();
}
