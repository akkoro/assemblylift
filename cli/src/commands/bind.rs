use std::fs;
use std::path;
use std::process;
use std::process::Stdio;

use clap::ArgMatches;

use crate::projectfs;
use crate::terraform;

pub fn command(matches: Option<&ArgMatches>) {
    let matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for bind command")
    };

    // TODO switch on platform; assume AWS for now since nothing else is supported
    
    terraform::extract();

    // run_terraform_init();
    // run_terraform_apply();
}
