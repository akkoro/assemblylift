use clap::ArgMatches;
use dialoguer::Confirm;

use crate::terraform;

pub fn command(_matches: Option<&ArgMatches>) {
    if Confirm::new()
        .with_prompt("Are you sure you want to destroy ALL provisioned infrastructure?\nThis is PERMANENT!")
        .interact()
        .unwrap()
    {
        terraform::commands::destroy()
    }
}
